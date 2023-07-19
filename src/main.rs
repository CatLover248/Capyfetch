use sysinfo::*;
use colored::Colorize;
fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("{}", " >> Capyfetch v0.0.1 <<".red());
    println!("{}", "> Disks".blue());
    for disk in sys.disks(){
        println!("  {:?}", disk);
    }
    println!("> Networks");
    for (interface_name, data) in sys.networks(){
        println!("  {}:{}/{}B", interface_name, data.received(), data.transmitted());
    }
    println!("{}", "> Memory".truecolor(100,25,250));
    println!("  total memory: {} bytes", sys.total_memory());
    println!("  used memory:  {} bytes", sys.used_memory());
    println!("  total swap:   {} bytes", sys.total_swap());
    println!("  used swap:    {} bytes", sys.used_swap());
    println!("{}", "> System info".purple());
    println!("  System name:           {:?}", sys.name());
    println!("  System kernel version: {:?}", sys.kernel_version());
    println!("  System OS version:     {:?}", sys.os_version());
    println!("  System host name:      {:?}", sys.host_name());
    // Art from -> https://www.asciiart.eu/nature/islands
    println!("{}", String::from("___                                                    ____
                                         v        _(    )
        _ ^ _                          v         (___(__)
       '_V/ `
       ' oX`
          X                            v
          X             -HELP!
          X                                                 .
          X                                                 |\
          X.a##a.   M                                       |_\
       .aa########a.>>                                    __|__
    .a################aa.                                   //
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                                                 David S. Issel").green());
}