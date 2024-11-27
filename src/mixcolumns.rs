pub fn mix_columns(state:Vec<Vec<u8>>)->Vec<Vec<u8>>{
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


pub fn inv_mix_columns(state:Vec<Vec<u8>>)->Vec<Vec<u8>>{
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