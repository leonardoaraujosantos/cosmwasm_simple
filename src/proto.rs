pub fn encode_protobuf_message(creator: &str, job_id: u64, results_json: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    
    // field 1: creator (string)
    encode_string_field(&mut buf, 1, creator);
    
    // field 2: job_id (uint64)
    encode_uint64_field(&mut buf, 2, job_id);
    
    // field 3: results_json (string)
    encode_string_field(&mut buf, 3, results_json);
    
    buf
}

fn encode_string_field(buf: &mut Vec<u8>, field_number: u32, value: &str) {
    // String fields are length-delimited (wire type 2)
    let wire_type = 2u32;
    let tag = (field_number << 3) | wire_type;
    
    // Write tag
    encode_varint(buf, tag as u64);
    
    // Write length of string
    encode_varint(buf, value.len() as u64);
    
    // Write string bytes
    buf.extend_from_slice(value.as_bytes());
}

fn encode_uint64_field(buf: &mut Vec<u8>, field_number: u32, value: u64) {
    // Varint fields use wire type 0
    let wire_type = 0u32;
    let tag = (field_number << 3) | wire_type;
    
    // Write tag
    encode_varint(buf, tag as u64);
    
    // Write value
    encode_varint(buf, value);
}

fn encode_varint(buf: &mut Vec<u8>, mut value: u64) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        buf.push(byte);
        if value == 0 {
            break;
        }
    }
}