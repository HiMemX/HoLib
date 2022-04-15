pub struct asset{
    pub length_with_padding: u32,
    pub offset: u32,
    pub length: u32,
    pub mid_bytes: u32, // You can ignore this
    pub id: u64,
    pub flag: u32,
    pub trail_bytes: u32, // You can ignore this
    pub data: Vec<u8>,
    pub name: String,
}