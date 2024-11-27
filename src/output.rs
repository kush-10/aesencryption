pub fn state_print(state:Vec<Vec<u8>>){
    for row in &state {
        for byte in row {
            print!("{:02X}", byte);
        }
        print!(" "); 
    }
    println!(); 

    for i in 0..4 {
        for col in &state {
            print!("{:?}\t", col[i]);
        }
        println!(); 
    }
    println!()
}


pub fn vec_to_hex_string(input: Vec<Vec<Vec<u8>>>) -> String {
    // Flatten the Vec<Vec<Vec<u8>>> into a single Vec<u8>
    let flattened: Vec<u8> = input.into_iter()
        .flat_map(|v2| v2.into_iter().flat_map(|v1| v1.into_iter()))
        .collect();
    
    // Convert each u8 in the flattened vector to a hexadecimal string
    flattened.iter()
        .map(|byte| format!("{:02X}", byte)) // Format each byte as two-digit hex
        .collect::<String>() // Collect all formatted strings into one string
}


pub fn cipher_print(ciphertext:Vec<Vec<Vec<u8>>>){
    for state in ciphertext{
        for row in &state {
            for byte in row {
                print!("{:02X}", byte);
            }
            print!(" "); 
        }
        println!(); 
    }
}