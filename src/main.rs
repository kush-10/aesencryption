pub mod output;
pub mod subbytes;
pub mod shiftrows;
pub mod mixcolumns;
        

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
         .map(|&a| subbytes::S_BOX[a as usize]) 
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
        if output {output::state_print(state.clone());}
        
        for round in 1..nr{
            if output {println!("-------{:?}-------",round);}
            
            let subed_state=subbytes::sub_bytes(state.clone());
            if output {println!("-------Sub-------",);output::state_print(subed_state.clone());}
            
            let shifted_state=shiftrows::shift_rows(subed_state);
            if output {println!("-------Shift-------",);output::state_print(shifted_state.clone());}
            
            let mixed_state = mixcolumns::mix_columns(shifted_state);
            if output {println!("-------Mix-------",);output::state_print(mixed_state.clone());}
            
            state = add_round_key(mixed_state, &expanded_key[round]);
            if output {println!("-------KeyAdd-------",);output::state_print(state.clone());}
        }
        let subed_state=subbytes::sub_bytes(state);
        if output {println!("-------Sub-------",);output::state_print(subed_state.clone());}
        
        let shifted_state=shiftrows::shift_rows(subed_state);
        if output {println!("-------Shift-------",);output::state_print(shifted_state.clone());}
        
        let final_state = add_round_key(shifted_state, &expanded_key[nr]);
        if output {println!("-------KeyAdd-------",);output::state_print(final_state.clone());}
        
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
        if output {output::state_print(state.clone());}
        
        for round in 1..(nr){
            if output{println!("-------{:?}-------",10-round);}

            let subed_state=subbytes::inv_sub_bytes(state.clone());
            if output {println!("-------Sub-------"); output::state_print(subed_state.clone());}
            
            let shifted_state=shiftrows::inv_shift_rows(subed_state);
            if output {println!("-------Shift-------"); output::state_print(shifted_state.clone());}
            
            let mixed_state = add_round_key(shifted_state, &expanded_key[nr-round]);
            if output {println!("-------KeyAdd-------"); output::state_print(mixed_state.clone());}
            
            state = mixcolumns::inv_mix_columns(mixed_state);
            if output {println!("-------Mix-------"); output::state_print(state.clone());}
        }
        let shifted_state=shiftrows::inv_shift_rows(state);
        if output {println!("-------Shift-------"); output::state_print(shifted_state.clone());}
        
        let subed_state=subbytes::inv_sub_bytes(shifted_state);
        if output {println!("-------Sub-------"); output::state_print(subed_state.clone());}
        
        let final_state = add_round_key(subed_state, &expanded_key[0]);
        if output {println!("-------KeyAdd-------"); output::state_print(final_state.clone());}
        
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
    output::cipher_print(ciphertext.clone());
    
    let cipherstring = output::vec_to_hex_string(ciphertext);
    
    let fplaintext = inv_cipher(&cipherstring, string_key,output);
    output::cipher_print(fplaintext.clone());

}
