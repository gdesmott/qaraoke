use std::io::prelude::*;
use std::io;
use std::borrow::Cow;
use std::cmp::min;

use lz4;
use ogg;

pub struct OggCdgCoder<R> {
    reader: R,
    packetsize: u8,
    // TODO: add keyframe support
    cur_frame: u64,
    last_keyframe: u64,
}

impl <R: Read> OggCdgCoder<R> {
    pub fn new(reader: R) -> Self {
        OggCdgCoder{
            reader: reader,
            packetsize: 75,
            cur_frame: 0,
            last_keyframe: 0,
        }
    }
}

impl <R: Read> ogg::BitstreamCoder for OggCdgCoder<R> {
    //type Frame = Frame;
    //type Error = io::Error;
    
    fn headers(&self) -> Vec<Vec<u8>> {
        let mut header = Vec::with_capacity(14);

        header.extend_from_slice(b"OggCDG\0\0");
        header.push(0); // Major
        header.push(0); // Minor
        header.push(Compression::LZ4 as u8); // LZ4
        header.push(self.packetsize-1);
        vec![header]
    }

    fn next_frame(&mut self) -> io::Result<Option<ogg::Packet>> {
        let mut input = Vec::with_capacity(self.packetsize as usize * 96);
        let mut output = Vec::new();
        let size = try!(self.reader.by_ref().take(self.packetsize as u64 * 96).read_to_end(&mut input));
        //let size = try!(self.reader.read(&mut input));
        if size % 96 != 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Incomplete sector read"));
        }

        if size == 0 {
            return Ok(None);
        }
        
        output.push(0);
        output.push((size / 96) as u8);

        let mut encoder = try!(lz4::EncoderBuilder::new()
                               .level(9)
                               .checksum(lz4::ContentChecksum::NoChecksum)
                               .build(output));
        
        try!(encoder.write_all(&input));
        let (output, result) = encoder.finish();
        try!(result);
        
        self.cur_frame += size as u64 / 96;

        Ok(Some(ogg::Packet{
            content: output,
            timestamp: self.cur_frame << 32 | self.last_keyframe,
        }))
    }

    fn map_granule(&self, granule: u64) -> u64 {
        (granule >> 32) * 1000_000 / 75
    }
}
#[derive(Copy,Clone,PartialEq,Debug)]
pub enum Compression {
    None,
    LZ4,
}

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum PacketType {
    Command,
    Keyframe,
    Other(u8),
}

impl PacketType {
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => PacketType::Command,
            1 => PacketType::Keyframe,
            _ => PacketType::Other(v),
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            PacketType::Command => 0,
            PacketType::Keyframe => 1,
            PacketType::Other(v) => v,
        }
    }
}


pub struct CdgHeader {
    pub compression: Compression,
    pub sectors_per_packet: usize,
}


impl CdgHeader {
    pub fn new() -> Self {
        CdgHeader{
            compression: Compression::LZ4,
            sectors_per_packet: 75, // 1s at a time
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let spp = min(self.sectors_per_packet - 1, 255) as u8;
        vec![
            b'O', b'g', b'g', b'C', b'D', b'G', 0, 0,
            0, 0, self.compression as u8, spp,
        ]
    }

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf[0..9] != *b"OggCDG\0\0\0" {
            return None;
        }
        
        let compression = match buf[10] {
            0 => Compression::None,
            1 => Compression::LZ4,
            _ => return None,
        };
        let spp = buf[11] as usize + 1;
        Some(CdgHeader{
            compression: compression,
            sectors_per_packet: spp,
        })
    }

    fn decompress_packet<'a>(&self, buf: &'a [u8]) -> io::Result<Cow<'a, [u8]>> {
        match self.compression {
            Compression::None => Ok(Cow::Borrowed(buf)),
            Compression::LZ4 => {
                use std::io::{Cursor, copy};
                let mut res = Vec::new();
                try!(copy(
                    &mut try!(lz4::Decoder::new(Cursor::new(buf))),
                    &mut res));
                Ok(Cow::Owned(res))
            }
        }        
    }

    /// Decode a packet from the middle of the stream
    pub fn decode_packet<'a>(&self, buf: &'a [u8]) -> Option<(PacketType, Cow<'a, [u8]>)> {
        let typ = PacketType::from_u8(buf[0]);
        self.decompress_packet(&buf[2..]).ok().map(|pkt| (typ, pkt))
    }
}
