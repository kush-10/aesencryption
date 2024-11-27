const S_BOX:[u8;256] =  
        [0x63 ,0x7c ,0x77 ,0x7b ,0xf2 ,0x6b ,0x6f ,0xc5 ,0x30 ,0x01 ,0x67 ,0x2b ,0xfe ,0xd7 ,0xab ,0x76
        ,0xca ,0x82 ,0xc9 ,0x7d ,0xfa ,0x59 ,0x47 ,0xf0 ,0xad ,0xd4 ,0xa2 ,0xaf ,0x9c ,0xa4 ,0x72 ,0xc0
        ,0xb7 ,0xfd ,0x93 ,0x26 ,0x36 ,0x3f ,0xf7 ,0xcc ,0x34 ,0xa5 ,0xe5 ,0xf1 ,0x71 ,0xd8 ,0x31 ,0x15
        ,0x04 ,0xc7 ,0x23 ,0xc3 ,0x18 ,0x96 ,0x05 ,0x9a ,0x07 ,0x12 ,0x80 ,0xe2 ,0xeb ,0x27 ,0xb2 ,0x75
        ,0x09 ,0x83 ,0x2c ,0x1a ,0x1b ,0x6e ,0x5a ,0xa0 ,0x52 ,0x3b ,0xd6 ,0xb3 ,0x29 ,0xe3 ,0x2f ,0x84
        ,0x53 ,0xd1 ,0x00 ,0xed ,0x20 ,0xfc ,0xb1 ,0x5b ,0x6a ,0xcb ,0xbe ,0x39 ,0x4a ,0x4c ,0x58 ,0xcf
        ,0xd0 ,0xef ,0xaa ,0xfb ,0x43 ,0x4d ,0x33 ,0x85 ,0x45 ,0xf9 ,0x02 ,0x7f ,0x50 ,0x3c ,0x9f ,0xa8
        ,0x51 ,0xa3 ,0x40 ,0x8f ,0x92 ,0x9d ,0x38 ,0xf5 ,0xbc ,0xb6 ,0xda ,0x21 ,0x10 ,0xff ,0xf3 ,0xd2
        ,0xcd ,0x0c ,0x13 ,0xec ,0x5f ,0x97 ,0x44 ,0x17 ,0xc4 ,0xa7 ,0x7e ,0x3d ,0x64 ,0x5d ,0x19 ,0x73
        ,0x60 ,0x81 ,0x4f ,0xdc ,0x22 ,0x2a ,0x90 ,0x88 ,0x46 ,0xee ,0xb8 ,0x14 ,0xde ,0x5e ,0x0b ,0xdb
        ,0xe0 ,0x32 ,0x3a ,0x0a ,0x49 ,0x06 ,0x24 ,0x5c ,0xc2 ,0xd3 ,0xac ,0x62 ,0x91 ,0x95 ,0xe4 ,0x79
        ,0xe7 ,0xc8 ,0x37 ,0x6d ,0x8d ,0xd5 ,0x4e ,0xa9 ,0x6c ,0x56 ,0xf4 ,0xea ,0x65 ,0x7a ,0xae ,0x08
        ,0xba ,0x78 ,0x25 ,0x2e ,0x1c ,0xa6 ,0xb4 ,0xc6 ,0xe8 ,0xdd ,0x74 ,0x1f ,0x4b ,0xbd ,0x8b ,0x8a
        ,0x70 ,0x3e ,0xb5 ,0x66 ,0x48 ,0x03 ,0xf6 ,0x0e ,0x61 ,0x35 ,0x57 ,0xb9 ,0x86 ,0xc1 ,0x1d ,0x9e
        ,0xe1 ,0xf8 ,0x98 ,0x11 ,0x69 ,0xd9 ,0x8e ,0x94 ,0x9b ,0x1e ,0x87 ,0xe9 ,0xce ,0x55 ,0x28 ,0xdf
        ,0x8c ,0xa1 ,0x89 ,0x0d ,0xbf ,0xe6 ,0x42 ,0x68 ,0x41 ,0x99 ,0x2d ,0x0f ,0xb0 ,0x54 ,0xbb ,0x16];


const INV_S_BOX:[u8;256]=
        [0x52 ,0x09 ,0x6a ,0xd5 ,0x30 ,0x36 ,0xa5 ,0x38 ,0xbf ,0x40 ,0xa3 ,0x9e ,0x81 ,0xf3 ,0xd7 ,0xfb
        ,0x7c ,0xe3 ,0x39 ,0x82 ,0x9b ,0x2f ,0xff ,0x87 ,0x34 ,0x8e ,0x43 ,0x44 ,0xc4 ,0xde ,0xe9 ,0xcb
        ,0x54 ,0x7b ,0x94 ,0x32 ,0xa6 ,0xc2 ,0x23 ,0x3d ,0xee ,0x4c ,0x95 ,0x0b ,0x42 ,0xfa ,0xc3 ,0x4e
        ,0x08 ,0x2e ,0xa1 ,0x66 ,0x28 ,0xd9 ,0x24 ,0xb2 ,0x76 ,0x5b ,0xa2 ,0x49 ,0x6d ,0x8b ,0xd1 ,0x25
        ,0x72 ,0xf8 ,0xf6 ,0x64 ,0x86 ,0x68 ,0x98 ,0x16 ,0xd4 ,0xa4 ,0x5c ,0xcc ,0x5d ,0x65 ,0xb6 ,0x92
        ,0x6c ,0x70 ,0x48 ,0x50 ,0xfd ,0xed ,0xb9 ,0xda ,0x5e ,0x15 ,0x46 ,0x57 ,0xa7 ,0x8d ,0x9d ,0x84
        ,0x90 ,0xd8 ,0xab ,0x00 ,0x8c ,0xbc ,0xd3 ,0x0a ,0xf7 ,0xe4 ,0x58 ,0x05 ,0xb8 ,0xb3 ,0x45 ,0x06
        ,0xd0 ,0x2c ,0x1e ,0x8f ,0xca ,0x3f ,0x0f ,0x02 ,0xc1 ,0xaf ,0xbd ,0x03 ,0x01 ,0x13 ,0x8a ,0x6b
        ,0x3a ,0x91 ,0x11 ,0x41 ,0x4f ,0x67 ,0xdc ,0xea ,0x97 ,0xf2 ,0xcf ,0xce ,0xf0 ,0xb4 ,0xe6 ,0x73
        ,0x96 ,0xac ,0x74 ,0x22 ,0xe7 ,0xad ,0x35 ,0x85 ,0xe2 ,0xf9 ,0x37 ,0xe8 ,0x1c ,0x75 ,0xdf ,0x6e
        ,0x47 ,0xf1 ,0x1a ,0x71 ,0x1d ,0x29 ,0xc5 ,0x89 ,0x6f ,0xb7 ,0x62 ,0x0e ,0xaa ,0x18 ,0xbe ,0x1b
        ,0xfc ,0x56 ,0x3e ,0x4b ,0xc6 ,0xd2 ,0x79 ,0x20 ,0x9a ,0xdb ,0xc0 ,0xfe ,0x78 ,0xcd ,0x5a ,0xf4
        ,0x1f ,0xdd ,0xa8 ,0x33 ,0x88 ,0x07 ,0xc7 ,0x31 ,0xb1 ,0x12 ,0x10 ,0x59 ,0x27 ,0x80 ,0xec ,0x5f
        ,0x60 ,0x51 ,0x7f ,0xa9 ,0x19 ,0xb5 ,0x4a ,0x0d ,0x2d ,0xe5 ,0x7a ,0x9f ,0x93 ,0xc9 ,0x9c ,0xef
        ,0xa0 ,0xe0 ,0x3b ,0x4d ,0xae ,0x2a ,0xf5 ,0xb0 ,0xc8 ,0xeb ,0xbb ,0x3c ,0x83 ,0x53 ,0x99 ,0x61
        ,0x17 ,0x2b ,0x04 ,0x7e ,0xba ,0x77 ,0xd6 ,0x26 ,0xe1 ,0x69 ,0x14 ,0x63 ,0x55 ,0x21 ,0x0c ,0x7d];
    
         
fn state_print(state:Vec<Vec<u8>>){
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


fn vec_to_hex_string(input: Vec<Vec<Vec<u8>>>) -> String {
    // Flatten the Vec<Vec<Vec<u8>>> into a single Vec<u8>
    let flattened: Vec<u8> = input.into_iter()
        .flat_map(|v2| v2.into_iter().flat_map(|v1| v1.into_iter()))
        .collect();
    
    // Convert each u8 in the flattened vector to a hexadecimal string
    flattened.iter()
        .map(|byte| format!("{:02X}", byte)) // Format each byte as two-digit hex
        .collect::<String>() // Collect all formatted strings into one string
}


fn cipher_print(ciphertext:Vec<Vec<Vec<u8>>>){
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

fn hex_string_to_blocks(plaintext:&str)-> Vec<Vec<u8>>{
    let char_vec: Vec<char> = plaintext.chars().collect();
    let denary = char_vec.chunks(2)
        .filter_map(|x| u8::from_str_radix(&String::from_iter(x).as_str(),16).ok())
        .collect::<Vec<_>>();
    let blocks = denary.chunks(16)
        .map(|x|x.to_vec())
        .collect::<Vec<_>>();
    return blocks;    
}

fn block_to_state(block:Vec<u8>)->Vec<Vec<u8>>{
    let state = block.chunks(4)
    .map(|x|x.to_vec())
    .collect::<Vec<_>>();

    return state;
}


fn sub_bytes(mut state:Vec<Vec<u8>>)-> Vec<Vec<u8>>{
    for word in state.iter_mut(){
        for byte in word.iter_mut(){
            *byte = S_BOX[*byte as usize];
        }
    }
    return state;
}

fn inv_sub_bytes(mut state:Vec<Vec<u8>>)-> Vec<Vec<u8>>{
    for word in state.iter_mut(){
        for byte in word.iter_mut(){
            *byte = INV_S_BOX[*byte as usize];
        }
    }
    return state;
}

fn shift_rows(state:Vec<Vec<u8>>)->Vec<Vec<u8>>{
    let mut shifted_state = vec![vec![0;4];4];

    for (c,column) in shifted_state.iter_mut().enumerate(){
        for (r,row) in column.iter_mut().enumerate(){
            *row = state[(c+r)%4][r];
        }
    }
    return shifted_state;

}

fn inv_shift_rows(state:Vec<Vec<u8>>)->Vec<Vec<u8>>{
    let mut shifted_state = vec![vec![0;4];4];

    for (c,column) in shifted_state.iter_mut().enumerate(){
        for (r,row) in column.iter_mut().enumerate(){
            *row = state[(4+c-r)%4][r];
        }
    }

    return shifted_state;

}

fn mix_columns(state:Vec<Vec<u8>>)->Vec<Vec<u8>>{
    let mult_matrix: Vec<Vec<u8>> = vec![vec![02,03,01,01],vec![01,02,03,01],vec![01,01,02,03],vec![03,01,01,02]];
    let mut mixed_state = vec![vec![0;4];4];
    
    let x_times = |mut x: u8| -> u8 {
        x = (x << 1) ^ if x > 127 { 27 } else { 0 };
        x
    };

    for i in 0..4{
        for j in 0..4{
            for k in 0..4{
                match mult_matrix[j][k]{
                    1 => mixed_state[i][j] ^= state[i][k], 
                    2 => mixed_state[i][j] ^= x_times(state[i][k]),
                    3 => mixed_state[i][j] ^= x_times(state[i][k]) ^state[i][k],
                    _ => println!("Fail"),
                }
            }
        }
    }
    return mixed_state;
}


fn inv_mix_columns(state:Vec<Vec<u8>>)->Vec<Vec<u8>>{
    let mult_matrix: Vec<Vec<u8>> = vec![vec![14,11,13,09],vec![09,14,11,13],vec![13,09,14,11],vec![11,13,09,14]];
    let mut mixed_state = vec![vec![0;4];4];
    
    let x_times = |mut x: u8| -> u8 {
        x = (x << 1) ^ if x > 127 { 27 } else { 0 };
        x
    };

    for i in 0..4{
        for j in 0..4{
            for k in 0..4{
                match mult_matrix[j][k]{
                    9 => mixed_state[i][j] ^= x_times(x_times(x_times(state[i][k]))) ^state[i][k],
                    11 => mixed_state[i][j] ^= x_times(x_times(x_times(state[i][k])))^ x_times(state[i][k]) ^state[i][k],
                    13 => mixed_state[i][j] ^= x_times(x_times(x_times(state[i][k])))^ x_times(x_times(state[i][k])) ^state[i][k],
                    14 => mixed_state[i][j] ^= x_times(x_times(x_times(state[i][k])))^ x_times(x_times(state[i][k])) ^x_times(state[i][k]),
                    _ => println!("Fail"),
                }
            }
        }
    }
    return mixed_state;
}


fn xor_vecs(vec1: &[u8], vec2: &[u8]) -> Vec<u8> {
    vec1.iter()
        .zip(vec2.iter())
        .map(|(&a, &b)| a ^ b)  
        .collect()
}




fn key_expansion(string_key:&str)->Vec<Vec<Vec<u8>>>{
    let nk=4;
    let nr=10;
    let rcon =vec![vec![1,0,0,0],vec![2,0,0,0],vec![4,0,0,0],vec![8,0,0,0],vec![16,0,0,0],vec![32,0,0,0],vec![64,0,0,0],vec![128,0,0,0],vec![27,0,0,0],vec![54,0,0,0]];
    let key = block_to_state(hex_string_to_blocks(string_key).into_iter().flatten().collect());
    let rotate_word = |mut x:Vec<u8>| -> Vec<u8> {
        x.rotate_left(1);
        x  
    };

    let sub_word = |x: Vec<u8>| -> Vec<u8> {
        x.iter()
         .map(|&a| S_BOX[a as usize]) 
         .collect()
    };

    let mut w :Vec<Vec<u8>> = key;

    for i in nk..=(4*nr+3){

        let mut temp = w[i-1].clone();
        if (i%nk) == 0{
            temp = xor_vecs(&sub_word(rotate_word(temp)) ,&rcon[(i/nk)-1]);
        }
        else if (nk>6) && (i%nk == 4){
            temp = sub_word(temp)
        }
        w.push(xor_vecs(&w[i-nk], &temp));
    }

    let key_expand = w.chunks(4)
    .map(|x|x.to_vec())
    .collect::<Vec<Vec<_>>>();

    return key_expand;
}


fn add_round_key(state: Vec<Vec<u8>>, key: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut modified_state = state;  // Make a mutable copy of state
    
    for (a, b) in modified_state.iter_mut().zip(key.iter()) {
        *a = xor_vecs(a, b);  // Dereference `a` to modify the vector in place
    }
    
    modified_state  // Return the modified state
}

fn cipher(plaintext:&str,string_key:&str,output:bool)->Vec<Vec<Vec<u8>>>{
    let nr=10;
    let expanded_key= key_expansion(string_key);
    let mut ciphertext: Vec<Vec<Vec<u8>>> = Vec::new();

    let blocks = hex_string_to_blocks(plaintext);
    
    for block in blocks{
        let mut state =block_to_state(block);
        
        state = add_round_key(state, &expanded_key[0]);
        if output {state_print(state.clone());}
        
        for round in 1..nr{
            if output {println!("-------{:?}-------",round);}
            
            let subed_state=sub_bytes(state.clone());
            if output {println!("-------Sub-------",);state_print(subed_state.clone());}
            
            let shifted_state=shift_rows(subed_state);
            if output {println!("-------Shift-------",);state_print(shifted_state.clone());}
            
            let mixed_state = mix_columns(shifted_state);
            if output {println!("-------Mix-------",);state_print(mixed_state.clone());}
            
            state = add_round_key(mixed_state, &expanded_key[round]);
            if output {println!("-------KeyAdd-------",);state_print(state.clone());}
        }
        let subed_state=sub_bytes(state);
        if output {println!("-------Sub-------",);state_print(subed_state.clone());}
        
        let shifted_state=shift_rows(subed_state);
        if output {println!("-------Shift-------",);state_print(shifted_state.clone());}
        
        let final_state = add_round_key(shifted_state, &expanded_key[nr]);
        if output {println!("-------KeyAdd-------",);state_print(final_state.clone());}
        
        ciphertext.push(final_state)
    }

    if output {println!("--------END---------");}
    return ciphertext;
}


fn inv_cipher(cipherstring:&str,string_key:&str,output:bool)->Vec<Vec<Vec<u8>>>{
    let nr=10;
    let expanded_key= key_expansion(string_key);
    let mut ciphertext: Vec<Vec<Vec<u8>>> = Vec::new();
    
    let blocks = hex_string_to_blocks(cipherstring);
    
    for block in blocks{
        let mut state =block_to_state(block);
       
        state = add_round_key(state, &expanded_key[nr]);
        if output {state_print(state.clone());}
        
        for round in 1..(nr){
            if output{println!("-------{:?}-------",10-round);}

            let subed_state=inv_sub_bytes(state.clone());
            if output {println!("-------Sub-------"); state_print(subed_state.clone());}
            
            let shifted_state=inv_shift_rows(subed_state);
            if output {println!("-------Shift-------"); state_print(shifted_state.clone());}
            
            let mixed_state = add_round_key(shifted_state, &expanded_key[nr-round]);
            if output {println!("-------KeyAdd-------"); state_print(mixed_state.clone());}
            
            state = inv_mix_columns(mixed_state);
            if output {println!("-------Mix-------"); state_print(state.clone());}
        }
        let shifted_state=inv_shift_rows(state);
        if output {println!("-------Shift-------"); state_print(shifted_state.clone());}
        
        let subed_state=inv_sub_bytes(shifted_state);
        if output {println!("-------Sub-------"); state_print(subed_state.clone());}
        
        let final_state = add_round_key(subed_state, &expanded_key[0]);
        if output {println!("-------KeyAdd-------"); state_print(final_state.clone());}
        
        ciphertext.push(final_state)
    }
    
    if output{println!("--------END---------");}
    return ciphertext;
}



fn main() {
    let plaintext =  "6BC1BEE22E409F96E93D7E117393172AAE2D8A571E03AC9C9EB76FAC45AF8E5130C81C46A35CE411E5FBC1191A0A52EFF69F2445DF4F9B17AD2B417BE66C3710";
    let string_key = "2B7E151628AED2A6ABF7158809CF4F3C";
    let output = false; 

    let ciphertext = cipher(plaintext, string_key,output);
    cipher_print(ciphertext.clone());
    
    let cipherstring = vec_to_hex_string(ciphertext);
    
    let fplaintext = inv_cipher(&cipherstring, string_key,output);
    cipher_print(fplaintext.clone());

}
