
pub fn shift_rows(state:Vec<Vec<u8>>)->Vec<Vec<u8>>{
    let mut shifted_state = vec![vec![0;4];4];

    for (c,column) in shifted_state.iter_mut().enumerate(){
        for (r,row) in column.iter_mut().enumerate(){
            *row = state[(c+r)%4][r];
        }
    }
    return shifted_state;

}

pub fn inv_shift_rows(state:Vec<Vec<u8>>)->Vec<Vec<u8>>{
    let mut shifted_state = vec![vec![0;4];4];

    for (c,column) in shifted_state.iter_mut().enumerate(){
        for (r,row) in column.iter_mut().enumerate(){
            *row = state[(4+c-r)%4][r];
        }
    }

    return shifted_state;

}
