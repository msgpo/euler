use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::env;

fn numeral_to_number(numeral: &char) -> u32{
    match *numeral {
        'M' => 1000,
        'D' => 500,
        'C' => 100,
        'L' => 50,
        'X' => 10,
        'V' => 5,
        'I' => 1,
        _ => 0,
    }
}

fn roman_to_arabic(roman: &str) -> u32 {
    let mut numerals = Vec::<u32>::new();
    for numeral in roman.chars() {
        numerals.push(numeral_to_number(&numeral));
    }
    numerals.reverse();
    let mut memo = 0;
    let mut acc = 0;
    for place in numerals {
        if place >= memo {
            acc += place;
        }
        else {
            acc -= place;
        }
        memo = place;
    }
    acc
}

fn place_writer(value: &u32, (ten, five, unit): (char, char, char)) -> String {
    let mut result = String::new();
    if *value == 9 {
        result.push(unit);
        result.push(ten);
    }
    else if *value >= 5 {
        result.push(five);
        for x in 0..(*value - 5){
            result.push(unit);
        }
    }
    else if *value == 4 {
        result.push(unit);
        result.push(five);
    }
    else {
        for x in 0..*value{
            result.push(unit);
        }
    }

    result
}

fn arabic_to_roman(arabic_src: &u32) -> String {
    let mut arabic = *arabic_src;

    let mut result = String::new();
    let M = arabic / 1000;
    for x in 0..M {
        result.push('M');
    }
    arabic = arabic - (M * 1000);

    let C = arabic / 100;
    result.push_str(place_writer(&C, ('M', 'D', 'C')).as_str());
    arabic = arabic - (C * 100);

    let X = arabic / 10;
    result.push_str(place_writer(&X, ('C', 'L', 'X')).as_str());
    arabic = arabic - (X * 10);

    let I = arabic;
    result.push_str(place_writer(&I, ('X', 'V', 'I')).as_str());

    result
}

fn compress_line(line: &str) -> u32 {
    // convert and rewrite, compare before and after, return the number of chars saved
    (line.len() - arabic_to_roman(&roman_to_arabic(line)).len()) as u32
}

fn main() {
    println!("Test Roman to Arabic:");
    let roms = ["MMXIV", "MCMXCIX", "XXV", "MDCLXVI", "MMMDCCCLXXXVIII"];
    for &r in &roms {
        println!("{:2$} = {}", r, roman_to_arabic(r), 15);
    }
    println!("Test Arabic to Roman:");
    let ars = [2014, 1999, 25, 1666, 3888];
    for &r in &ars {
        println!("{:2$} = {:2$}", r, arabic_to_roman(&r), 4);
    }

    let args: Vec<_> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let file = BufReader::new(&f);

    let mut acc = 0;
    for line in file.lines() {
        acc += compress_line(line.unwrap().as_str());
    }
    println!("Total chars saved: {}", acc);
}
