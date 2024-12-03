use std::fmt::Display;

use crate::{buffer::Buffer, error::ErrorType, header::Mode};

#[derive(Debug)]
pub struct SideInfo {
    pub main_data_begin: u16,
    pub private_bits: u8,
    pub scfsi: u8,
    pub granules: [Granule; 2],
}

#[derive(Debug, Default)]
pub struct Granule {
    pub channels: Vec<ChannelInfo>,
}

#[derive(Debug, Default)]
pub struct ChannelInfo {
    pub part_23_length: u16,
    pub big_values: u16,
    pub global_gain: u8,
    pub scalefac_compress: u8,
    pub windows_switching: bool,
    pub block_type: u8,
    pub mixed_block_flag: bool,
    pub table_select: [u8; 3],
    pub subblock_gain: [u8; 3],
    pub region_count: [u8; 3],
    pub preflag: bool,
    pub scalefac_scale: bool,
    pub count1_table_select: bool,
}

impl Display for ChannelInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let part_23_length = format!("\t part_23_length: {:#2x}\n", self.part_23_length);
        let big_values = format!("\t big_values: {:#2x}\n", self.big_values);
        let global_gain = format!("\t global_gain: {:#8b}\n", self.global_gain);
        let scalefac_compress = format!("\t scalefac_compress: {:#8b}\n", self.scalefac_compress);
        let windows_switching = format!("\t windows_switching: {}\n", self.windows_switching);
        let block_type = format!("\t block_type: {:#2b}\n", self.block_type);
        let mixed_block_flag = format!("\t mixed_block_flag: {}\n", self.mixed_block_flag);
        let table_select = format!(
            "\t table_select: [{:#8b}, {:#8b}, {:#8b}]\n",
            self.table_select[0], self.table_select[1], self.table_select[2]
        );
        let subblock_gain = format!(
            "\t subblock_gain: [{:#8b}, {:#8b}, {:#8b}]\n",
            self.subblock_gain[0], self.subblock_gain[1], self.subblock_gain[2]
        );
        let region_count = format!(
            "\t region_count: [{:#8b}, {:#8b}, {:#8b}]\n",
            self.region_count[0], self.region_count[1], self.region_count[2]
        );
        let preflag = format!("\t preflag: {}\n", self.preflag);
        let scalefac_scale = format!("\t scalefac_scale: {}\n", self.scalefac_scale);
        let count1_table_select = format!("\t count1_table_select: {}\n", self.count1_table_select);

        write!(f, "{part_23_length}{big_values}{global_gain}{scalefac_compress}{windows_switching}{block_type}{mixed_block_flag}{table_select}{subblock_gain}{region_count}{preflag}{scalefac_scale}{count1_table_select}")
    }
}

impl ChannelInfo {
    #[inline]
    fn new() -> Self {
        Self::default()
    }
}

impl Display for Granule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut granule_result = Vec::new();

        for (i, channel) in self.channels.iter().enumerate() {
            granule_result.push(format!("\nChannel {i}: \n{channel}\n"));
        }

        write!(f, "{}", granule_result.join("\n"))
    }
}

impl Granule {
    #[inline]
    fn new() -> Self {
        Self::default()
    }
}

impl Display for SideInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let main_data_begin = format!("main_data_begin: {:#x}\n", self.main_data_begin);
        let private_bits = format!("private_bits: {:#8b}\n", self.private_bits);
        let scfsi = format!("scfsi: {:#8b}\n", self.scfsi);
        let granule0 = format!("granule 0:\n{}\n", self.granules[0]);
        let granule1 = format!("granule 1:\n{}\n", self.granules[1]);

        write!(
            f,
            "{main_data_begin}{private_bits}{scfsi}\n{granule0}{granule1}\n"
        )
    }
}

impl SideInfo {
    pub fn create_from_buffer(buffer: &mut Buffer, mode: &Mode) -> Result<Self, ErrorType> {
        let is_mono = *mode == Mode::SingleChannel;

        let main_data_begin = buffer.get_bits(9).unwrap() as u16;
        let private_bits: u8 = buffer.get_bits(if is_mono { 5 } else { 3 }).unwrap() as u8;
        let scfsi = buffer.get_bits(if is_mono { 4 } else { 8 }).unwrap() as u8;

        let nch = if is_mono { 1 } else { 2 };
        let mut granules: [Granule; 2] = [Granule::new(), Granule::new()];
        let mut part_23_sum: usize = 0;

        for granule in granules.iter_mut() {
            for _ in 0..nch {
                let mut channel = ChannelInfo::new();

                channel.part_23_length = buffer.get_bits(12).unwrap() as u16;
                part_23_sum += channel.part_23_length as usize;

                channel.big_values = buffer.get_bits(9).unwrap() as u16;

                if channel.big_values > 288 {
                    return Err(ErrorType::BigValuesOutOfRange);
                }

                channel.global_gain = buffer.get_bits(8).unwrap() as u8;
                channel.scalefac_compress = buffer.get_bits(4).unwrap() as u8;
                channel.windows_switching = buffer.get_bits(1).unwrap() == 1;

                if channel.windows_switching {
                    channel.block_type = buffer.get_bits(2).unwrap() as u8;
                    if channel.block_type == 0 {
                        return Err(ErrorType::BlockTypeForbidden);
                    }

                    channel.mixed_block_flag = buffer.get_bits(1).unwrap() == 1;

                    for i in 0..2 {
                        channel.table_select[i] = buffer.get_bits(5).unwrap() as u8;
                    }

                    for i in 0..3 {
                        channel.subblock_gain[i] = buffer.get_bits(3).unwrap() as u8;
                    }
                } else {
                    for i in 0..3 {
                        channel.table_select[i] = buffer.get_bits(5).unwrap() as u8;
                    }

                    channel.region_count[0] = buffer.get_bits(4).unwrap() as u8;
                    channel.region_count[1] = buffer.get_bits(3).unwrap() as u8;
                    channel.region_count[2] = 255;
                }

                let bits = buffer.get_bits(3).unwrap() as u8;
                channel.preflag = bits & 4 == 4;
                channel.scalefac_scale = bits & 2 == 2;
                channel.count1_table_select = bits & 1 == 1;

                granule.channels.push(channel);
            }
        }

        if part_23_sum + buffer.pos > buffer.total_bits + main_data_begin as usize * 8 {
            return Err(ErrorType::Overflow);
        }

        Ok(SideInfo {
            main_data_begin,
            private_bits,
            scfsi,
            granules,
        })
    }
}
