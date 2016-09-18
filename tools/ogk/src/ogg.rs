use std::io::{self,Write};
use std::collections;
use rand;

// Low-level interface

const CRC_LOOKUP : [u32;256] = [
  0x00000000,0x04c11db7,0x09823b6e,0x0d4326d9,
  0x130476dc,0x17c56b6b,0x1a864db2,0x1e475005,
  0x2608edb8,0x22c9f00f,0x2f8ad6d6,0x2b4bcb61,
  0x350c9b64,0x31cd86d3,0x3c8ea00a,0x384fbdbd,
  0x4c11db70,0x48d0c6c7,0x4593e01e,0x4152fda9,
  0x5f15adac,0x5bd4b01b,0x569796c2,0x52568b75,
  0x6a1936c8,0x6ed82b7f,0x639b0da6,0x675a1011,
  0x791d4014,0x7ddc5da3,0x709f7b7a,0x745e66cd,
  0x9823b6e0,0x9ce2ab57,0x91a18d8e,0x95609039,
  0x8b27c03c,0x8fe6dd8b,0x82a5fb52,0x8664e6e5,
  0xbe2b5b58,0xbaea46ef,0xb7a96036,0xb3687d81,
  0xad2f2d84,0xa9ee3033,0xa4ad16ea,0xa06c0b5d,
  0xd4326d90,0xd0f37027,0xddb056fe,0xd9714b49,
  0xc7361b4c,0xc3f706fb,0xceb42022,0xca753d95,
  0xf23a8028,0xf6fb9d9f,0xfbb8bb46,0xff79a6f1,
  0xe13ef6f4,0xe5ffeb43,0xe8bccd9a,0xec7dd02d,
  0x34867077,0x30476dc0,0x3d044b19,0x39c556ae,
  0x278206ab,0x23431b1c,0x2e003dc5,0x2ac12072,
  0x128e9dcf,0x164f8078,0x1b0ca6a1,0x1fcdbb16,
  0x018aeb13,0x054bf6a4,0x0808d07d,0x0cc9cdca,
  0x7897ab07,0x7c56b6b0,0x71159069,0x75d48dde,
  0x6b93dddb,0x6f52c06c,0x6211e6b5,0x66d0fb02,
  0x5e9f46bf,0x5a5e5b08,0x571d7dd1,0x53dc6066,
  0x4d9b3063,0x495a2dd4,0x44190b0d,0x40d816ba,
  0xaca5c697,0xa864db20,0xa527fdf9,0xa1e6e04e,
  0xbfa1b04b,0xbb60adfc,0xb6238b25,0xb2e29692,
  0x8aad2b2f,0x8e6c3698,0x832f1041,0x87ee0df6,
  0x99a95df3,0x9d684044,0x902b669d,0x94ea7b2a,
  0xe0b41de7,0xe4750050,0xe9362689,0xedf73b3e,
  0xf3b06b3b,0xf771768c,0xfa325055,0xfef34de2,
  0xc6bcf05f,0xc27dede8,0xcf3ecb31,0xcbffd686,
  0xd5b88683,0xd1799b34,0xdc3abded,0xd8fba05a,
  0x690ce0ee,0x6dcdfd59,0x608edb80,0x644fc637,
  0x7a089632,0x7ec98b85,0x738aad5c,0x774bb0eb,
  0x4f040d56,0x4bc510e1,0x46863638,0x42472b8f,
  0x5c007b8a,0x58c1663d,0x558240e4,0x51435d53,
  0x251d3b9e,0x21dc2629,0x2c9f00f0,0x285e1d47,
  0x36194d42,0x32d850f5,0x3f9b762c,0x3b5a6b9b,
  0x0315d626,0x07d4cb91,0x0a97ed48,0x0e56f0ff,
  0x1011a0fa,0x14d0bd4d,0x19939b94,0x1d528623,
  0xf12f560e,0xf5ee4bb9,0xf8ad6d60,0xfc6c70d7,
  0xe22b20d2,0xe6ea3d65,0xeba91bbc,0xef68060b,
  0xd727bbb6,0xd3e6a601,0xdea580d8,0xda649d6f,
  0xc423cd6a,0xc0e2d0dd,0xcda1f604,0xc960ebb3,
  0xbd3e8d7e,0xb9ff90c9,0xb4bcb610,0xb07daba7,
  0xae3afba2,0xaafbe615,0xa7b8c0cc,0xa379dd7b,
  0x9b3660c6,0x9ff77d71,0x92b45ba8,0x9675461f,
  0x8832161a,0x8cf30bad,0x81b02d74,0x857130c3,
  0x5d8a9099,0x594b8d2e,0x5408abf7,0x50c9b640,
  0x4e8ee645,0x4a4ffbf2,0x470cdd2b,0x43cdc09c,
  0x7b827d21,0x7f436096,0x7200464f,0x76c15bf8,
  0x68860bfd,0x6c47164a,0x61043093,0x65c52d24,
  0x119b4be9,0x155a565e,0x18197087,0x1cd86d30,
  0x029f3d35,0x065e2082,0x0b1d065b,0x0fdc1bec,
  0x3793a651,0x3352bbe6,0x3e119d3f,0x3ad08088,
  0x2497d08d,0x2056cd3a,0x2d15ebe3,0x29d4f654,
  0xc5a92679,0xc1683bce,0xcc2b1d17,0xc8ea00a0,
  0xd6ad50a5,0xd26c4d12,0xdf2f6bcb,0xdbee767c,
  0xe3a1cbc1,0xe760d676,0xea23f0af,0xeee2ed18,
  0xf0a5bd1d,0xf464a0aa,0xf9278673,0xfde69bc4,
  0x89b8fd09,0x8d79e0be,0x803ac667,0x84fbdbd0,
  0x9abc8bd5,0x9e7d9662,0x933eb0bb,0x97ffad0c,
  0xafb010b1,0xab710d06,0xa6322bdf,0xa2f33668,
  0xbcb4666d,0xb8757bda,0xb5365d03,0xb1f740b4];

pub struct Packet {
    pub content: Vec<u8>,
    pub timestamp: u64,
}

bitflags!{
    pub flags PageFlags: u8 {
        const PAGE_CTD = 1,
        const PAGE_BOS = 2,
        const PAGE_EOS = 4,
    }
}

pub struct Page {
    pub flags: PageFlags,
    pub granule_position: u64,
    pub stream_serial: u32,
    pub page_sequence: u32,
    /// The last element is the number of elements used
    pub segment_table: Vec<u8>,
    pub content: Vec<u8>,
}

fn crc_compute(mut value: u32, block: &[u8]) -> u32 {
    for b in block {
        value = value << 8 ^ CRC_LOOKUP[((value >> 24) as u8 ^ b) as usize];
    }
    return value;
}

impl Page {
    pub fn new(stream_serial: u32, sequence: u32) -> Self {
        Page{
            flags: PageFlags::empty(),
            granule_position: !0,
            stream_serial: stream_serial,
            page_sequence: sequence,
            segment_table: Vec::new(),
            content: Vec::new(),
        }
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        use byteorder::{LittleEndian,ByteOrder};
        let mut header = [0;27];
        header[0..5].copy_from_slice(b"OggS\0");
        header[5] = self.flags.bits();
        LittleEndian::write_u64(&mut header[6..14], self.granule_position);
        LittleEndian::write_u32(&mut header[14..18], self.stream_serial);
        LittleEndian::write_u32(&mut header[18..22], self.page_sequence);
        // leave checksum in 22..26 blank
        header[26] = self.segment_table.len() as u8;

        let mut crc = 0;
        crc = crc_compute(crc, &header);
        crc = crc_compute(crc, &self.segment_table);
        crc = crc_compute(crc, &self.content);
        LittleEndian::write_u32(&mut header[22..26], crc);

        try!(writer.write_all(&header));
        try!(writer.write_all(&self.segment_table));
        try!(writer.write_all(&self.content));
        Ok(())
    }

    fn add_segment(&mut self, segment: &[u8]) {
        assert!(segment.len() < 256);
        assert!(self.segment_table.len() < 255);
        self.segment_table.push(segment.len() as u8);
        self.content.extend(segment);
    }

    fn has_space(&self) -> bool {
        self.segment_table.len() < 255
    }

    /// Add the part of packet that starts at offset.  If there is
    /// already data in this packet, offset MUST be 0.
    ///
    /// # Returns
    ///
    /// None: The packet was completely written and the next packet
    /// may be written immediately.
    ///
    /// Some(n): There was insufficent space to write the packet, and
    /// the same packet should be added to a new frame, passing n as
    /// offset.
    fn add_packet(&mut self, packet: &Packet, offset: usize) -> Option<usize> {
        let mut offset = offset;
        let plen = packet.content.len();
        if offset != 0 {
            assert!(self.segment_table.len() == 0);
            assert!(!self.flags.intersects(PAGE_BOS));
            self.flags |= PAGE_CTD;
        }
        while plen - offset > 254 && self.has_space() {
            // Write a 255-byte segment
            self.add_segment(&packet.content[offset..offset+255]);
            offset += 255;
        }
        if self.has_space() {
            // write final chunk
            assert!(plen - offset < 255);
            self.add_segment(&packet.content[offset..]);
            self.granule_position = packet.timestamp;
            return None;
        } else {
            return Some(offset);
        }
    }

    pub fn content_size(&self) -> usize {
        return self.segment_table.iter().map(|x| *x as usize).sum();
    }
}

// Page packer
pub struct PagePacker {
    page_queue : collections::VecDeque<Page>,

    // This is only none when the stream is closed
    active_page: Option<Page>,

    stream_serial: u32,
    page_sequence: u32,
}

impl PagePacker {
    pub fn new(serial: u32) -> Self {
        let mut start_page = Page::new(serial, 0);
        start_page.flags |= PAGE_BOS;
        PagePacker{
            page_queue: collections::VecDeque::new(),
            active_page: Some(start_page),
            stream_serial: serial,
            page_sequence: 0,
        }
    }

    // Immediately emit the current page and prepare the next one
    pub fn emit(&mut self) {
        // Emit a page and get the next page ready
        if let Some(page) = self.active_page.take() {
            let eos = page.flags.intersects(PAGE_EOS);
            self.page_queue.push_back(page);
            if !eos {
                self.page_sequence += 1;
                self.active_page = Some(Page::new(self.stream_serial, self.page_sequence));
            }
        }
    }

    fn get_active(&mut self) -> &mut Page {
        match self.active_page.as_mut() {
            None => panic!("Attempted to add packets to a closed stream"),
            Some(page) => page,
        }
    }

    pub fn add_packet(&mut self, packet: &Packet) {
        let mut offset = Some(0);
        while let Some(off) = offset {
            offset = self.get_active().add_packet(packet, off);
            if offset.is_some() {
                self.emit()
            }
        }
        // TODO: Limit page size?
        //if self.get_active().content_size() > 8192 {
        //self.emit()
        //}
    }

    pub fn close(&mut self) {
        self.get_active().flags |= PAGE_EOS;
        self.emit();
        assert!(self.active_page.is_none());
    }

    pub fn is_closed(&self) -> bool {
        self.active_page.is_none()
    }

    pub fn peek_next(&self) -> Option<&Page> {
        self.page_queue.front()
    }

    pub fn take_next(&mut self) -> Option<Page> {
        self.page_queue.pop_front()
    }
}

pub trait BitstreamCoder {
    fn headers(&self) -> Vec<Vec<u8>>;
    fn next_frame(&mut self) -> io::Result<Option<Packet>>;

    /// Map a granule position to an absolute timestamp in µs
    fn map_granule(&self, u64) -> u64;
}

struct MuxStream {
    bitstream: Box<BitstreamCoder>,
    packer: PagePacker,
}

impl MuxStream {
    fn pump(&mut self) -> io::Result<()> {
        while self.packer.peek_next().is_none() && !self.packer.is_closed() {
            if let Some(frame) = try!(self.bitstream.next_frame()) {
                self.packer.add_packet(&frame);
                // TODO: Emit early for lower bufferring needs
            } else {
                self.packer.close();
            }
        }
        Ok(())
    }

    fn is_live(&self) -> bool {
        self.packer.peek_next().is_some() || !self.packer.is_closed()
    }
}

// Muxer
pub struct OgkMux {
    streams: Vec<MuxStream>,
}

impl OgkMux {
    pub fn new() -> Self {
        OgkMux{
            streams: Vec::new(),
        }
    }

    pub fn add_stream(&mut self, stream: Box<BitstreamCoder>) {
        let serial = rand::random();
        self.streams.push(MuxStream{
            bitstream: stream,
            packer: PagePacker::new(serial),
        })
    }

    pub fn write_to<W: io::Write>(&mut self, mut w: W) -> io::Result<()> {
        // Write headers...
        for stream in self.streams.iter_mut() {
            let headers = stream.bitstream.headers();
            let first_packet = try!(stream.bitstream.next_frame());
            let header_count = headers.len();
            let mut it = headers.into_iter();
            stream.packer.add_packet(&Packet{content: it.next().expect("Streams must contain at least one header"), timestamp: 0});
            // Produce additional headers, if any
            if header_count > 1 {
                stream.packer.emit();
                for header in it {
                    stream.packer.add_packet(&Packet{content: header, timestamp: 0});
                }
            }
            // Close the stream
            if let Some(packet) = first_packet {
                stream.packer.emit();
                stream.packer.add_packet(&packet);
            } else {
                stream.packer.close();
            }
            try!(stream.packer.take_next().unwrap().write_to(&mut w));
        }

        // Write the remaining header packets
        for stream in self.streams.iter_mut() {
            while let Some(frame) = stream.packer.take_next() {
                try!(frame.write_to(&mut w));
            }
        }

        for stream in self.streams.iter_mut() {
            try!(stream.pump());
        }

        loop {
            self.streams.retain(MuxStream::is_live);
            if self.streams.is_empty() {
                break;
            }
            if let Some(stream) = self.streams.iter_mut().min_by_key(|stream| stream.packer.peek_next().unwrap().granule_position) {
                try!(stream.packer.take_next().unwrap().write_to(&mut w));
                try!(stream.pump());
            } else {
                panic!("This should be unreachable");
            }
        }
        Ok(())
    }
}