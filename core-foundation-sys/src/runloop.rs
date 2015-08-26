// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc::c_void;

use array::CFArrayRef;
use base::{Boolean, CFIndex, CFTypeID, CFAllocatorRef, CFOptionFlags, CFHashCode, mach_port_t};
use date::{CFAbsoluteTime, CFTimeInterval};
use string::CFStringRef;

#[repr(C)]
struct __CFRunLoop;

pub type CFRunLoopRef = *const __CFRunLoop;

#[repr(C)]
struct __CFRunLoopSource;

pub type CFRunLoopSourceRef = *const __CFRunLoopSource;

#[repr(C)]
struct __CFRunLoopObserver;

pub type CFRunLoopObserverRef = *const __CFRunLoopObserver;

// Reasons for CFRunLoopRunInMode() to Return
pub const kCFRunLoopRunFinished: i32      = 1;
pub const kCFRunLoopRunStopped: i32       = 2;
pub const kCFRunLoopRunTimedOut: i32      = 3;
pub const kCFRunLoopRunHandledSource: i32 = 4;

// Run Loop Observer Activities
//typedef CF_OPTIONS(CFOptionFlags, CFRunLoopActivity) {
pub type CFRunLoopActivity = CFOptionFlags;
pub const kCFRunLoopEntry: CFOptionFlags         = 1 << 0;
pub const kCFRunLoopBeforeTimers: CFOptionFlags  = 1 << 1;
pub const kCFRunLoopBeforeSources: CFOptionFlags = 1 << 2;
pub const kCFRunLoopBeforeWaiting: CFOptionFlags = 1 << 5;
pub const kCFRunLoopAfterWaiting: CFOptionFlags  = 1 << 6;
pub const kCFRunLoopExit: CFOptionFlags          = 1 << 7;
pub const kCFRunLoopAllActivities: CFOptionFlags = 0x0FFFFFFF;

#[repr(C)]
pub struct CFRunLoopSourceContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: extern "C" fn (info: *const c_void) -> *const c_void,
    pub release: extern "C" fn (info: *const c_void),
    pub copyDescription: extern "C" fn (info: *const c_void) -> CFStringRef,
    pub equal: extern "C" fn (info1: *const c_void, info2: *const c_void) -> Boolean,
    pub hash: extern "C" fn (info: *const c_void) -> CFHashCode,
    pub schedule: extern "C" fn (info: *const c_void, rl: CFRunLoopRef, mode: CFStringRef),
    pub cancel: extern "C" fn (info: *const c_void, rl: CFRunLoopRef, mode: CFStringRef),
    pub perform: extern "C" fn (info: *const c_void),
}

#[repr(C)]
pub struct CFRunLoopSourceContext1 {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: extern "C" fn (info: *const c_void) -> *const c_void,
    pub release: extern "C" fn (info: *const c_void),
    pub copyDescription: extern "C" fn (info: *const c_void) -> CFStringRef,
    pub equal: extern "C" fn (info1: *const c_void, info2: *const c_void) -> Boolean,
    pub hash: extern "C" fn (info: *const c_void) -> CFHashCode,
    // note that the following two fields are platform dependent in the C header, the ones here are for OS X
    pub getPort: extern "C" fn (info: *mut c_void) -> mach_port_t,
    pub perform: extern "C" fn (msg: *mut c_void, size: CFIndex, allocator: CFAllocatorRef, info: *mut c_void) -> *mut c_void,
}

#[repr(C)]
pub struct CFRunLoopObserverContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: extern "C" fn (info: *const c_void) -> *const c_void,
    pub release: extern "C" fn (info: *const c_void),
    pub copyDescription: extern "C" fn (info: *const c_void) -> CFStringRef,
}

pub type CFRunLoopObserverCallBack = extern "C" fn (observer: CFRunLoopObserverRef, activity: CFRunLoopActivity, info: *mut c_void);

#[repr(C)]
pub struct CFRunLoopTimerContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: extern "C" fn (info: *const c_void) -> *const c_void,
    pub release: extern "C" fn (info: *const c_void),
    pub copyDescription: extern "C" fn (info: *const c_void) -> CFStringRef,
}

pub type CFRunLoopTimerCallBack = extern "C" fn (timer: CFRunLoopTimerRef, info: *mut c_void);

#[repr(C)]
struct __CFRunLoopTimer;

pub type CFRunLoopTimerRef = *const __CFRunLoopTimer;

extern {
    /*
     * CFRunLoop.h
     */
    pub static kCFRunLoopDefaultMode: CFStringRef;
    pub static kCFRunLoopCommonModes: CFStringRef;
    pub fn CFRunLoopGetTypeID() -> CFTypeID;
    pub fn CFRunLoopGetCurrent() -> CFRunLoopRef;
    pub fn CFRunLoopGetMain() -> CFRunLoopRef;
    pub fn CFRunLoopCopyCurrentMode(rl: CFRunLoopRef) -> CFStringRef;
    pub fn CFRunLoopCopyAllModes(rl: CFRunLoopRef) -> CFArrayRef;
    pub fn CFRunLoopAddCommonMode(rl: CFRunLoopRef, mode: CFStringRef);
    pub fn CFRunLoopGetNextTimerFireDate(rl: CFRunLoopRef, mode: CFStringRef) -> CFAbsoluteTime;
    pub fn CFRunLoopRun();
    pub fn CFRunLoopRunInMode(mode: CFStringRef, seconds: CFTimeInterval, returnAfterSourceHandled: Boolean) -> i32;
    pub fn CFRunLoopIsWaiting(rl: CFRunLoopRef) -> Boolean;
    pub fn CFRunLoopWakeUp(rl: CFRunLoopRef);
    pub fn CFRunLoopStop(rl: CFRunLoopRef);
    // fn CFRunLoopPerformBlock(rl: CFRunLoopRef, mode: CFTypeRef, block: void (^)(void));
    pub fn CFRunLoopContainsSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef) -> Boolean;
    pub fn CFRunLoopAddSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef);
    pub fn CFRunLoopRemoveSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef);
    pub fn CFRunLoopContainsObserver(rl: CFRunLoopRef, observer: CFRunLoopObserverRef, mode: CFStringRef) -> Boolean;
    pub fn CFRunLoopAddObserver(rl: CFRunLoopRef, observer: CFRunLoopObserverRef, mode: CFStringRef);
    pub fn CFRunLoopRemoveObserver(rl: CFRunLoopRef, observer: CFRunLoopObserverRef, mode: CFStringRef);
    pub fn CFRunLoopContainsTimer(rl: CFRunLoopRef, timer: CFRunLoopTimerRef, mode: CFStringRef) -> Boolean;
    pub fn CFRunLoopAddTimer(rl: CFRunLoopRef, timer: CFRunLoopTimerRef, mode: CFStringRef);
    pub fn CFRunLoopRemoveTimer(rl: CFRunLoopRef, timer: CFRunLoopTimerRef, mode: CFStringRef);

    pub fn CFRunLoopSourceGetTypeID() -> CFTypeID;
    pub fn CFRunLoopSourceCreate(allocator: CFAllocatorRef, order: CFIndex, context: *mut CFRunLoopSourceContext) -> CFRunLoopSourceRef;
    pub fn CFRunLoopSourceGetOrder(source: CFRunLoopSourceRef) -> CFIndex;
    pub fn CFRunLoopSourceInvalidate(source: CFRunLoopSourceRef);
    pub fn CFRunLoopSourceIsValid(source: CFRunLoopSourceRef) -> Boolean;
    pub fn CFRunLoopSourceGetContext(source: CFRunLoopSourceRef, context: *mut CFRunLoopSourceContext);
    pub fn CFRunLoopSourceSignal(source: CFRunLoopSourceRef);

    pub fn CFRunLoopObserverGetTypeID() -> CFTypeID;
    pub fn CFRunLoopObserverCreate(allocator: CFAllocatorRef, activities: CFOptionFlags, repeats: Boolean, order: CFIndex, callout: CFRunLoopObserverCallBack, context: *mut CFRunLoopObserverContext) -> CFRunLoopObserverRef;
    // fn CFRunLoopObserverCreateWithHandler(allocator: CFAllocatorRef, activities: CFOptionFlags, repeats: Boolean, order: CFIndex, block: void (^) (CFRunLoopObserverRef observer, CFRunLoopActivity activity)) -> CFRunLoopObserverRef;
    pub fn CFRunLoopObserverGetActivities(observer: CFRunLoopObserverRef) -> CFOptionFlags;
    pub fn CFRunLoopObserverDoesRepeat(observer: CFRunLoopObserverRef) -> Boolean;
    pub fn CFRunLoopObserverGetOrder(observer: CFRunLoopObserverRef) -> CFIndex;
    pub fn CFRunLoopObserverInvalidate(observer: CFRunLoopObserverRef);
    pub fn CFRunLoopObserverIsValid(observer: CFRunLoopObserverRef) -> Boolean;
    pub fn CFRunLoopObserverGetContext(observer: CFRunLoopObserverRef, context: *mut CFRunLoopObserverContext);

    pub fn CFRunLoopTimerGetTypeID() -> CFTypeID;
    pub fn CFRunLoopTimerCreate(allocator: CFAllocatorRef, fireDate: CFAbsoluteTime, interval: CFTimeInterval, flags: CFOptionFlags, order: CFIndex, callout: CFRunLoopTimerCallBack, context: *mut CFRunLoopTimerContext) -> CFRunLoopTimerRef;
    // fn CFRunLoopTimerCreateWithHandler(allocator: CFAllocatorRef, fireDate: CFAbsoluteTime, interval: CFTimeInterval, flags: CFOptionFlags, order: CFIndex, block: void (^) (CFRunLoopTimerRef timer)) -> CFRunLoopTimerRef;
    pub fn CFRunLoopTimerGetNextFireDate(timer: CFRunLoopTimerRef) -> CFAbsoluteTime;
    pub fn CFRunLoopTimerSetNextFireDate(timer: CFRunLoopTimerRef, fireDate: CFAbsoluteTime);
    pub fn CFRunLoopTimerGetInterval(timer: CFRunLoopTimerRef) -> CFTimeInterval;
    pub fn CFRunLoopTimerDoesRepeat(timer: CFRunLoopTimerRef) -> Boolean;
    pub fn CFRunLoopTimerGetOrder(timer: CFRunLoopTimerRef) -> CFIndex;
    pub fn CFRunLoopTimerInvalidate(timer: CFRunLoopTimerRef);
    pub fn CFRunLoopTimerIsValid(timer: CFRunLoopTimerRef) -> Boolean;
    pub fn CFRunLoopTimerGetContext(timer: CFRunLoopTimerRef, context: *mut CFRunLoopTimerContext);
    pub fn CFRunLoopTimerGetTolerance(timer: CFRunLoopTimerRef) -> CFTimeInterval;
    pub fn CFRunLoopTimerSetTolerance(timer: CFRunLoopTimerRef, tolerance: CFTimeInterval);
}
