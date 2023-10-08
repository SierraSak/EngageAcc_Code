#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use engage::*;

#[repr(C)]
#[unity::class("App", "AccessoryData")]
pub struct app_accessorydata {
    structbase: [u8; 0x10],
    pub aid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub help: &'static Il2CppString,
    pub name_m: &'static Il2CppString,
    pub help_m: &'static Il2CppString,
    pub name_f: &'static Il2CppString,
    pub help_f: &'static Il2CppString,
    pub first: bool,
    pub amiibo: bool,
    pub condition_cid: &'static Il2CppString,
    pub condition_gender: i32,
    pub condition_skills: &'static [Il2CppString; 0],
    pub gid: &'static Il2CppString,
    pub asset: &'static Il2CppString,
    pub price: i32,
    pub iron: i32,
    pub steel: i32,
    pub silver: i32,
    pub mask: i32,
    pub kind: i32,
    pub god_data: u64,
    pub flag_name: &'static Il2CppString,
    
    // ...
}

#[repr(C)]
#[unity::class("App", "UnitAccessoryList")]
pub struct unit_accessory_list {
    pub unit_accessory_array: &'static mut Il2CppArray<&'static Il2CppObject<unit_accessory>>,
}

#[repr(C)]
#[unity::class("App", "UnitAccessory")]
pub struct unit_accessory {
    pub index: i32,
}


#[unity::hook("App", "UnitAccessoryList", "get_count")]
pub fn app_unitaccessorylist_getcount(this: &mut Il2CppObject<unit_accessory_list>, method_info: OptionalMethod,)-> i32{
    return 15;
}


#[unity::hook("App", "AccessoryData", "OnBuild")]
pub fn onbuild_accessory_data_hook(this: &mut Il2CppObject<app_accessorydata>, method_info: OptionalMethod,)
{
    call_original!(this, method_info);
    if this.mask > 8
    {
        match this.mask{
            16=>this.kind = 4,
            32=>this.kind = 5,
            64=>this.kind = 6,
            128=>this.kind = 7,
            256=>this.kind = 8,
            512=>this.kind = 9,
            1024=>this.kind = 10,
            2048=>this.kind = 11,
            4096=>this.kind = 12,
            8192=>this.kind = 13,
            16384=>this.kind = 14,
            32768=>this.kind = 15,
            _=>this.kind = 1,
        }
    }
}

//Currently not compiling
//cannot borrow data in dereference of `Array<&unity::prelude::Il2CppObject<unit_accessory>>` as mutable
//#[unity::hook("App", "UnitAccessoryList", "Clear")]
//pub fn clear_UnitAccessoryList_hook(this: &mut Il2CppObject<unit_accessory_list>, method_info: OptionalMethod,)
//{
//    //call_original!(this, method_info);
//    let mut i = 0;
//    while i < this.unit_accessory_array.len()
//    {
//        this.unit_accessory_array[i].index = 0;
//        i += 1;
//    }
//}

//#[skyline::hook(offset = 0x1F62090)]
//pub fn add_UnitAccessoryList_hook(this: &mut Il2CppObject<unit_accessory_list>, accessory: &mut Il2CppObject<app_accessorydata>, index: i32, method_info: OptionalMethod,)
//{
    //let mut i = 0;
    //let mut equipped_acc_index = 0;
    //let mut acc_check;
    //while i < this.unit_accessory_array.len()
    //{
        //equipped_acc_index = this.unit_accessory_array[i].index;
        //acc_check = TryGet Accessory XML data from acc_index
        //if acc_check != 0 and accessory.mask == acc_check.mask
        //{
            //this.unit_accessory_array[i].index = 0;
        //}
    //}
    //i = 0;
    //if index < 0
    //{
        //equipped_acc_index = this.unit_accessory_array[i].index;
        //if equipped_acc_index == 0
        //{
            //this.unit_accessory_array[i].index = accessory.super.super.super.Index;
        //}
        //i += 1;
    //}
    //else if index < this.unit_accessory_array.len()
    //{
        //this.unit_accessory_array[index].index = accessory.super.super.super.Index;
        //i += 1;
    //}
//}

//#[unity::hook("App", "UnitAccessoryList", "IsExist")]
//pub fn unitaccessorylist_is_exist_hook(this: &mut Il2CppObject<unit_accessory_list>, accessory: &mut Il2CppObject<app_accessorydata>, method_info: OptionalMethod,)-> bool
//{
    //if accessory == 0x0{
    //return false;
    //}
    //else{
        //let mut i = 0;
        //let mut acc_index = 0;
        //let mut acc_check;
        //while i < this.unit_accessory_array.len()
        //{
            //acc_index = this.unit_accessory_array[i].index;
            //acc_check = TryGet Accessory XML data from acc_index
            //if acc_check == accessory{
                //return true;
            //}
            //i += 1;
        //}
        //return false;
//}



#[unity::hook("App", "UnitAccessoryList", ".ctor")]
pub fn unitaccessorylist_ctor_hook(this: &mut Il2CppObject<unit_accessory_list>, method_info: OptionalMethod,)
{
    call_original!(this, method_info);
    
    this.unit_accessory_array[0] = unit_accessory::instantiate().map(|acc| {acc.index = 0; acc}).unwrap(); 
    this.unit_accessory_array[1] = unit_accessory::instantiate().map(|acc| {acc.index = 0; acc}).unwrap(); 
    this.unit_accessory_array[2] = unit_accessory::instantiate().map(|acc| {acc.index = 0; acc}).unwrap(); 
    this.unit_accessory_array[3] = unit_accessory::instantiate().map(|acc| {acc.index = 0; acc}).unwrap(); 
    this.unit_accessory_array[4] = unit_accessory::instantiate().map(|acc| {acc.index = 0; acc}).unwrap(); 
    this.unit_accessory_array[5] = unit_accessory::instantiate().map(|acc| {acc.index = 0; acc}).unwrap(); 
    this.unit_accessory_array[6] = unit_accessory::instantiate().map(|acc| {acc.index = 0; acc}).unwrap(); 
    this.unit_accessory_array[7] = unit_accessory::instantiate().map(|acc| {acc.index = 0; acc}).unwrap(); 
    this.unit_accessory_array[8] = unit_accessory::instantiate().map(|acc| {acc.index = 0; acc}).unwrap(); 
    this.unit_accessory_array[9] = unit_accessory::instantiate().map(|acc| {acc.index = 0; acc}).unwrap(); 
    this.unit_accessory_array[10] = unit_accessory::instantiate().map(|acc| {acc.index = 0; acc}).unwrap(); 
    this.unit_accessory_array[11] = unit_accessory::instantiate().map(|acc| {acc.index = 0; acc}).unwrap(); 
    this.unit_accessory_array[12] = unit_accessory::instantiate().map(|acc| {acc.index = 0; acc}).unwrap(); 
    this.unit_accessory_array[13] = unit_accessory::instantiate().map(|acc| {acc.index = 0; acc}).unwrap(); 
    this.unit_accessory_array[14] = unit_accessory::instantiate().map(|acc| {acc.index = 0; acc}).unwrap();
    this.unit_accessory_array[15] = unit_accessory::instantiate().map(|acc| {acc.index = 0; acc}).unwrap(); 
}

#[unity::hook("App", "AccessoryShopChangeRoot", "OnSelectMenuItem")]
pub fn onselectmenuitem_accessory_data_hook(this: &Il2CppObject<()>, accessory_data: &mut Il2CppObject<app_accessorydata>, method_info: OptionalMethod,)
{
    call_original!(this, accessory_data, method_info)
}

#[skyline::main(name = "TestProject")]
pub fn main() {
    skyline::install_hooks!(unitaccessorylist_ctor_hook, onbuild_accessory_data_hook, app_unitaccessorylist_getcount);
    skyline::patching::Patch::in_text(0x01f61c00).bytes(&[0x01, 0x02, 0x80, 0x52]).expect("Couldn’t patch that shit for some reasons");
    skyline::patching::Patch::in_text(0x027b5d70).bytes(&[0xDF, 0x3E, 0x00, 0x71]).expect("Couldn’t patch that shit for some reasons");
    skyline::patching::Patch::in_text(0x027b5d8c).bytes(&[0xDF, 0x42, 0x00, 0x71]).expect("Couldn’t patch that shit for some reasons");
    //skyline::patching::Patch::in_text(0x027bffcc).bytes(&[0x1F, 0x20, 0x03, 0xD5]).expect("Couldn’t patch that shit for some reasons");
}
