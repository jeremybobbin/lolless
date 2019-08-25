extern crate termion;

use termion::{
    cursor,
    color,
    clear,
};

use std::{
    io::{
        self,
        BufReader,
        Read,
        Write,
        BufRead,
        LineWriter,
        SeekFrom,
        Seek,
    },

    env::{
        args,
        var
    },
    fs::{
        self,
        File
    },
    f64::consts::PI,
    thread,
    time::Duration
};

const LIMIT: usize = 1000;

fn main() {

    let mut reader: Box<dyn Read> = if let Some(path) = args().nth(1) {
        let file = File::open(&path) 
            .expect(&format!("Could not open '{}'.", &path));
        Box::new(file)
    } else {
        Box::new(io::stdin())
    };

    let mut input = String::with_capacity(LIMIT);

    reader.take(LIMIT as u64)
        .read_to_string(&mut input)
        .expect("Dang.");

    let mut reader = io::Cursor::new(input);
    let mut writer = io::stdout();

    let mut positive: Vec<f64> = (0..)
        .into_iter()
        .map(|i| i as f64 / 500.)
        .take_while(|&i| i < 1.)
        .collect();

    let full: Vec<f64> = positive
        .iter()
        .map(|&i| i * -1.)
        .rev()
        .chain(positive.iter().map(|&i| i))
        .collect();

    let forward = full.iter();

    let mut backward = forward
        .clone()
        .rev();
    
    let mut primary = forward
        .clone()
        .chain(backward.clone())
        .cycle();

    let mut secondary = backward
        .chain(forward)
        .cycle();

    let iteration = primary
        .zip(secondary);

    print!("{}{}", clear::All, cursor::Hide);

    for (&a, &b) in iteration {
        print!("{}", cursor::Goto(1,1));
        lolcat(&mut reader, &mut writer, (a, b))
            .expect("Error writing to screen");

        reader.seek(SeekFrom::Start(0))
            .expect("Error seeking in file");

        thread::sleep(Duration::from_millis(10));
    }
}

fn lolcat<R: Read, W: Write>(reader: R, writer: W, multipliers: (f64, f64)) -> io::Result<()>  {
    let (xm, ym) = multipliers;

    let mut reader = BufReader::new(reader);
    let mut writer = LineWriter::new(writer);

    let mut y = 0;
    let mut buf = String::new();

    while let Ok(res) = reader.read_line(&mut buf) {
        if res == 0 {
            break;
        }

        if buf.ends_with('\n') {
            buf.pop();
            if buf.ends_with('\r') {
                buf.pop();
            }
        }

        for (x, c) in buf.chars().enumerate() {
            if !c.is_ascii() {
                continue;
            }
            let i = (y as f64 * ym) + (x as f64 * xm);

            let r = (((i                    ).sin() * 127.) + 128.) as u8;
            let g = (((i + ((2. * PI) / 3.) ).sin() * 127.) + 128.) as u8;
            let b = (((i + ((4. * PI) / 3.) ).sin() * 127.) + 128.) as u8;

            write!(&mut writer, "{}{}", color::Rgb(r, g, b).fg_string(), c)?;
        }
        write!(&mut writer, "\n")?;
        y += 1;
        buf.clear();
    }
    writer.flush()?;
    Ok(())
}