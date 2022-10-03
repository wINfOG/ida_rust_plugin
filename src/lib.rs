use autocxx::prelude::*;
use autocxx::subclass::prelude::*;

use libc::c_char;

//use std::ptr;

include_cpp! {
    //header
    #include "ida.hpp"
    #include "idp.hpp"
    #include "loader.hpp"
    #include "kernwin.hpp"

    safety!(unsafe_ffi)
    
    //function
    generate!("vmsg")
    
    //class
    generate!("plugin_t")
    generate!("plugmod_t")

    //subclass
    subclass!("plugmod_t", Plugin_ctx)
    
    
}

//write log in ida-console
#[macro_export]
macro_rules! idamsg {
    ($($t:tt)*) => {{
        _ida_msg_impl(format!($($t)*));
    }};
}
fn _ida_msg_impl(show_msg: String) {
    unsafe{
        ffi::vmsg((show_msg+"\r\n\0").as_ptr() as *const c_char, "\0".as_ptr() as *mut c_char);
    }
}

/*
-----origin C++ struct
class plugin_t
{
public:
  int version;                  
  int flags;                    ///< \ref PLUGIN_
  plugmod_t *(idaapi *init)(void);  ///< Initialize plugin - returns a pointer to plugmod_t
  void (idaapi *term)(void);      ///< Terminate plugin. This function will be called
  bool (idaapi *run)(size_t arg); ///< Invoke plugin.
  const char *comment;            ///< Long comment about the plugin.
  const char *help;               ///< Multiline help about the plugin
  const char *wanted_name;        ///< The preferred short name of the plugin
  const char *wanted_hotkey;      ///< The preferred hotkey to run the plugin
};
*/
const IDA_WANTED_NAME:&'static [u8;19] = &*b"hello rust plugin!\x00";

const IDA_NULL_PTR:[u8;8] = *b"\x00\x00\x00\x00\x00\x00\x00\x00";

#[repr(C)]
pub struct IDA_plugin_t{
    version: u32,
    flags: u32,
    init: fn() -> *mut ffi::Plugin_ctxCpp ,
    term: Option<fn()>,
    run: Option<fn(usize) -> bool>,
    comment: [u8;8], //todo
    help:  [u8;8], //todo
    wanted_name:&'static [u8;IDA_WANTED_NAME.len()], //todo
    wanted_hotkey: [u8;8], //todo
}



const IDP_INTERFACE_VERSION: u32 = 700;

const PLUGIN_MOD: u32 = 0x0001;
const PLUGIN_DRAW: u32 = 0x0002;
const PLUGIN_SEG: u32 = 0x0004;
const PLUGIN_UNL: u32 = 0x0008;
const PLUGIN_HIDE: u32 = 0x0010;
const PLUGIN_DBG: u32 = 0x0020;
const PLUGIN_PROC: u32 = 0x0040;
const PLUGIN_FIX: u32 = 0x0080;
const PLUGIN_MULTI: u32 = 0x0100;

#[no_mangle]
pub static PLUGIN: IDA_plugin_t = IDA_plugin_t {
    ///< Should be equal to #IDP_INTERFACE_VERSION; 与ida接口版本相同
    version: IDP_INTERFACE_VERSION,
    flags: PLUGIN_UNL | PLUGIN_MULTI,
    init: log_rust_msg,
    term: None,
    run: None,
    comment: IDA_NULL_PTR,
    help: IDA_NULL_PTR,
    wanted_name:IDA_WANTED_NAME,
    wanted_hotkey: IDA_NULL_PTR,
};


#[no_mangle]
fn log_rust_msg() -> *mut ffi::Plugin_ctxCpp {
    idamsg!("hello {}", "call-by-rust");
    return Plugin_ctx::default_cpp_owned().into_raw();
}


// ------- origin c++ class
// struct plugmod_t
// {
//   size_t owner = 0;     // internal info used by the kernel
//   size_t reserved = 0;  // for the future

//   /// Invoke the plugin.
//   virtual bool idaapi run(size_t arg) = 0;

//   /// Helper function to hook event listeners.
//   bool hook_event_listener(
//         hook_type_t hook_type,
//         event_listener_t *cb,
//         int hkcb_flags=0)
//   {
//     return ::hook_event_listener(hook_type, cb, this, hkcb_flags);
//   }

//   /// Virtual destructor.
//   virtual ~plugmod_t() {}
// };
//
// ------- origin c++ implement
// struct plugin_ctx_t : public plugmod_t
// {
//   virtual bool idaapi run(size_t) override;
// };
// bool idaapi plugin_ctx_t::run(size_t)
// {
//   //msg("Hello, world! (rust)\n");
//   return true;
// }
// plugmod_t *idaapi init()
// {
//   return new plugin_ctx_t;
// }

#[is_subclass(superclass("plugmod_t"))]
#[derive(Default)]
pub struct Plugin_ctx;

impl ffi::plugmod_t_methods for Plugin_ctx {
    fn run(&mut self, _arg:usize) -> bool {
        idamsg!("start—up && 中文测试");
        return true;
    }
}
