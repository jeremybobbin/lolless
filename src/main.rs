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

const LIMIT: usize = 10000;
const STEP: f64 = 0.01;
const WIDTH: f64 = 5.0;

fn main() {

    // Heap allocated READER
    let mut reader: Box<dyn Read> = if let Some(path) = args().nth(1) {
        let file = File::open(&path) 
            .expect(&format!("Could not open '{}'.", &path));
        Box::new(file)
    } else {
        Box::new(io::stdin())
    };


    // TODO: Only read current (WIDTH * SIZE) of terminal
    let mut input = String::with_capacity(LIMIT);

    reader.take(LIMIT as u64)
        .read_to_string(&mut input)
        .expect("Dang.");

    let mut reader = io::Cursor::new(input);
    let mut writer = io::stdout();


    let iteration = (1..)
        .map(|x| (x as f64) * STEP)
        .take_while(|x| *x < 2. * PI)
        .map(|x| (x.sin() / WIDTH, x.cos() / WIDTH))
        .cycle();

    print!("{}{}", clear::All, cursor::Hide);

    for (a, b) in iteration {
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
