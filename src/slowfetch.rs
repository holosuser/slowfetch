use sheller::run;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
fn fflush(){
    io::stdout().flush().unwrap();
}
fn main(){
    //os
    thread::sleep(Duration::from_secs(3));
    print!("os :: ");
    fflush();
    thread::sleep(Duration::from_secs(3));
    run!("tr -d '\"' < /etc/os-release | grep PRETTY_NAME | cut -b 13-");
    //kernel
    thread::sleep(Duration::from_secs(3));
    print!("kernel :: ");
    fflush();
    thread::sleep(Duration::from_secs(3));
    run!("uname -rs");
    //shell
    thread::sleep(Duration::from_secs(3));
    print!("shell :: ");
    fflush();
    thread::sleep(Duration::from_secs(3));
    run!("echo $SHELL");
    //cpu
    thread::sleep(Duration::from_secs(3));
    print!("cpu :: ");
    fflush();
    thread::sleep(Duration::from_secs(3));
    run!("cat /proc/cpuinfo | grep 'model name' | head -n1 | cut -b 14-");
    //gpu
    thread::sleep(Duration::from_secs(3));
    print!("gpu :: ");
    fflush();
    thread::sleep(Duration::from_secs(3));
    run!("lspci | grep -i vga | cut -b 36- | sed 's/ *(rev.*)//'");
}
