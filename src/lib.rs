extern crate libc;
use std::ffi::{CStr, CString};

pub type jint = i32;
pub type jdouble = f64;


pub enum _jobject {}
pub type jobject = *mut _jobject;

pub enum _jclass {}
pub type jclass = *mut _jclass;

pub enum _jmethodID {}
pub type jmethodID = *mut _jmethodID;

pub struct jvalue(jobject);

#[repr(C)]
pub struct JNINativeInterface {
    reserved0: *const libc::c_void,
    reserved1: *const libc::c_void,
    reserved2: *const libc::c_void,
    reserved3: *const libc::c_void,
    GetVersion: extern "C" fn(*mut JNIEnv) -> jint,
    DefineClass: *const libc::c_void,
    FindClass: *const libc::c_void,
    FromReflectedMethod: *const libc::c_void,
    FromReflectedField: *const libc::c_void,
    ToReflectedMethod: *const libc::c_void,
    GetSuperclass: *const libc::c_void,
    IsAssignableFrom: *const libc::c_void,
    ToReflectedField: *const libc::c_void,
    Throw: *const libc::c_void,
    ThrowNew: *const libc::c_void,
    ExceptionOccurred: *const libc::c_void,
    ExceptionDescribe: *const libc::c_void,
    ExceptionClear: *const libc::c_void,
    FatalError: *const libc::c_void,
    PushLocalFrame: *const libc::c_void,
    PopLocalFrame: *const libc::c_void,
    NewGlobalRef: *const libc::c_void,
    DeleteGlobalRef: *const libc::c_void,
    DeleteLocalRef: *const libc::c_void,
    IsSameObject: *const libc::c_void,
    NewLocalRef: *const libc::c_void,
    EnsureLocalCapacity: *const libc::c_void,
    AllocObject: *const libc::c_void,
    NewObject: *const libc::c_void,
    NewObjectV: *const libc::c_void,
    NewObjectA: *const libc::c_void,
    GetObjectClass: extern "C" fn (*mut JNIEnv, obj: jobject) -> jclass,
    IsInstanceOf: *const libc::c_void,
    GetMethodId: extern "C" fn (*mut JNIEnv, class: jclass, name: *const libc::c_char, sig: *const libc::c_char) -> jmethodID,
    CallObjectMethod: *const libc::c_void,
    CallObjectMethodV: *const libc::c_void,
    CallObjectMethodA: extern "C" fn (*mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jobject
}

#[repr(C)]
pub struct JNIEnv {
    functions: *mut JNINativeInterface
}

impl JNIEnv {
    fn _this(&mut self) -> *mut Self {
        self as *mut Self
    }

    fn GetMethodID(&mut self, class: jclass, name: &str, sig: &str) -> jmethodID {
        let name = CString::new(name).unwrap();
        let sig = CString::new(sig).unwrap();


        unsafe { ((*self.functions).GetMethodId)(self._this(), class, name.as_ptr(), sig.as_ptr()) }
    }

    fn GetObjectClass(&mut self, obj: jobject) -> jclass {
        unsafe { ((*self.functions).GetObjectClass)(self._this(), obj) }
    }

    fn GetVersion(&mut self) -> jint {
        unsafe { ((*self.functions).GetVersion)(self._this()) }
    }

    fn CallObjectMethod(&mut self, obj: jobject, methodID: jmethodID, args: &[jvalue]) -> jobject {
        unsafe { 
            let pargs = args.as_ptr();
            ((*self.functions).CallObjectMethodA)(self._this(), obj, methodID, pargs)
        }
    }
}

#[no_mangle]
pub extern fn Java_HelloJNI_sayHello(env: *mut JNIEnv, this: jobject) {
    println!("Hello from Rust!");
}

#[no_mangle]
pub extern fn Java_HelloJNI_average(env: *mut JNIEnv, this: jobject, a: jint, b: jint) -> jdouble {
    let da = a as jdouble;
    let db = b as jdouble;
    (da+db)/2.0
}

#[no_mangle]
pub unsafe extern fn Java_HelloJNI_callFromRust(env: *mut JNIEnv, this: jobject) {
    //let class = GetObjectClass(env, this);
    println!("We will call a Java method from within Rust...");
    println!("Version: {}", (*env).GetVersion());

    let thisClass = (*env).GetObjectClass(this);
    println!("This class: {:?}", thisClass);

    let methodID = (*env).GetMethodID(thisClass, "showFromJava", "()V");
    println!("Method ID: {:?}", methodID);

    (*env).CallObjectMethod(this, methodID, &[]);
}
