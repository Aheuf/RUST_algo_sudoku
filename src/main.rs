fn main() {
    let l = 0;
    let c = 3;
    let n = 2;
    let grid: [[u8;9];9] = [
        [5,3,0,0,7,0,0,0,0],
        [6,0,0,1,9,5,0,0,0],
        [0,9,8,0,0,0,0,6,0],
        [8,0,0,0,6,0,0,0,3],
        [4,0,0,8,0,3,0,0,1],
        [7,0,0,0,2,0,0,0,6],
        [0,6,0,0,0,0,2,8,0],
        [0,0,0,4,1,9,0,0,5],
        [0,0,0,0,8,0,0,7,9]
    ];

    print_grid(&grid);
    print!("{:?}",is_possible(&grid, l, c, n));
}

fn print_grid(grid: &[[u8;9];9]){
    let mut j = 1;
    for x in grid {
        let mut i = 1;
        for y in x {
            if *y == 0 {
                print!("   ");
            } else {
                print!(" {:?} ",y);
            }
            if i==3 || i==6 {
                print!("|");
            }
            i += 1;
        }
        println!("");
        if j == 3 || j == 6 {
            println!("---------|---------|---------");
        }
        j += 1;
    }
    println!("");
}

fn is_possible(grid: &[[u8;9];9], l: u8, c: u8, n: u8) -> bool {
    let mut contain: bool;

    //trouver la ligne et l'explorer
    contain = grid[l as usize].contains(&n);

    //trouver la colonne et l'explorer
    for x in grid {
        if x[c as usize] == n{
            contain = true;
            break;
        }
    }
    //trouver la case et l'explorer
    if l >= 0 && l <= 2 {

    } else if l >= 3 && l <= 5 {

    } else if l >= 6 && l <= 8 {

    }
    //renvoyer la réponse
    if contain == true {
        return false;
    } else {
        return true;
    }
}