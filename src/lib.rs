extern crate jni;
mod log;
use jni::objects::{JClass, JString};
use jni::sys::jint;
use jni::JNIEnv;
use log::Logger;

#[no_mangle]
pub extern "system" fn Java_com_xph_example_Logger_logInit(
    mut env: JNIEnv,
    _class: JClass,
    jpath: JString,
) -> jint {
    let path: String = env
        .get_string(&jpath)
        .expect("Couldn't get serial number string!")
        .into();

    match Logger::setup_logger(path) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_xph_example_Logger_logInfo(
    mut env: JNIEnv,
    _class: JClass,
    jmsg: JString,
) {
    let msg: String = env
        .get_string(&jmsg)
        .expect("Couldn't get java string!")
        .into();
    Logger::info(&msg);
}

#[no_mangle]
pub extern "system" fn Java_com_xph_example_Logger_logDebug(
    mut env: JNIEnv,
    _class: JClass,
    jmsg: JString,
) {
    let msg: String = env
        .get_string(&jmsg)
        .expect("Couldn't get java string!")
        .into();
    Logger::debug(&msg);
}

#[no_mangle]
pub extern "system" fn Java_com_xph_example_Logger_logWarn(
    mut env: JNIEnv,
    _class: JClass,
    jmsg: JString,
) {
    let msg: String = env
        .get_string(&jmsg)
        .expect("Couldn't get java string!")
        .into();
    Logger::warn(&msg);
}

#[no_mangle]
pub extern "system" fn Java_com_xph_example_Logger_logError(
    mut env: JNIEnv,
    _class: JClass,
    jmsg: JString,
) {
    let msg: String = env
        .get_string(&jmsg)
        .expect("Couldn't get java string!")
        .into();
    Logger::error(&msg);
}
