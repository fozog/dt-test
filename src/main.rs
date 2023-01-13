extern crate fdt_rs;
use fdt_rs::index::{DevTreeIndex};
use fdt_rs::base::*;
use crate::fdt_rs::prelude::PropReader;
use crate::fdt_rs::prelude::FallibleIterator;

mod dt;

use crate::dt::{read_two_items};
use crate::dt::DeviceTree;

// Place a device tree image into the rust binary and
// align it to a 32-byte boundary by using a wrapper struct.
#[repr(align(4))] struct _Wrapper<T>(T);
pub const FDT: &[u8] = &_Wrapper(*include_bytes!("../tests/rpi4.dtb")).0;

fn main() {
    // Initialize the devtree using an &[u8] array.
    println!("Working...");

    let mut scratchpad_backend: Vec<u8>;
        
    let devt = unsafe {
        let size = DevTree::read_totalsize(FDT).unwrap();
        let buf = &FDT[..size];
        set_dt_from_raw_parts!(buf.as_ptr(), scratchpad_backend) 
    };

    let mem_node= devt.get_node_by_name("memory").unwrap();
    let mut reg_prop = devt.get_prop_by_name(&mem_node, "reg").unwrap();
    let reg = read_two_items(reg_prop, devt.acells, devt.scells);
    println!("memmory:");
    for r in reg {
        println!("    {:#012x}-{:#012x}", r.base, r.base + r.size);
    }

    let memreserve_node= devt.get_node_by_path("/").unwrap();
    reg_prop = devt.get_prop_by_name(&memreserve_node, "memreserve").unwrap();
    let memreserve = read_two_items(reg_prop, 1, 1);
    println!("memory reservations:");
    for rsv in devt.devtree.reserved_entries()
    {
        println!("    {:#012x}-{:#012x}", u64::from(rsv.address), u64::from(rsv.address) + u64::from(rsv.size));
    }
    for r in memreserve {
        println!("    {:#012x}-{:#012x}", r.base, r.base + r.size);
    }

    let chosen_node= devt.get_node_by_name("chosen").unwrap();
    let stdout_prop = devt.get_prop_by_name(&chosen_node, "stdout-path").unwrap();
    let stdout = stdout_prop.iter_str().next().unwrap().unwrap();
    println!("stdout={}", stdout);

    let stdout_node= devt.get_node_by_path(stdout);
    let stdout = match stdout_node {
        None => panic!("stdout-path not found"),
        Some(ref c) => c
    };
    
    /*
    let path = dt::to_path(stdout);
    println!("stdout-path={}", path);
    */

    let compatible_prop = devt.get_prop_by_name(&stdout, "compatible").unwrap();
    let mut compatible_strings = compatible_prop.iter_str();
    while let Some(s) = compatible_strings.next().unwrap() {
        println!("   compatible={}", s);
    }

    let mmio = devt.parse_mmio(&stdout);

    for m in mmio {
        println!("    mmio={:#012x}-{:#012x}", m.base, m.base+m.size);
    }

}