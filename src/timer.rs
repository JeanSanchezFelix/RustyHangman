// ::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
//  timer.rs — POSIX Timers via SIGALRM (setitimer-based)
// ::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::

use std::sync::{Arc, Mutex, OnceLock, atomic::{AtomicBool, Ordering}};

use libc;

use crate::game::GameState;

// Global flag set by the SIGALRM handler when time expires.
// AtomicBool is safe to write from a signal handler.
// `pub` so integration tests can read/reset it.
pub static TIMED_OUT: AtomicBool = AtomicBool::new(false);

// Shared GameState reachable from inside the signal handler.
// Signal handlers have a fixed C signature (no user-data pointer),
// so the only way to pass state to them is through a module-level global.
static STATE: OnceLock<Arc<Mutex<GameState>>> = OnceLock::new();


// ----------------------------------------------------------
// TODO 3-A: Implement `RegisterSignalHandler`
//   Register `SigalrmHandler` as the handler for SIGALRM.
// ----------------------------------------------------------
pub fn RegisterSignalHandler(state: Arc<Mutex<GameState>>) {
    let _ = STATE.set(state);
    // println!("::::TASK 3-A:::: Registering SIGALRM handler...");
    // Hint: unsafe {}
    todo!("TODO 3-A: register SigalrmHandler for SIGALRM, store state Arc in STATE")
}

// ----------------------------------------------------------
// TODO 3-B: Implement `SigalrmHandler`
//   This extern "C" function is called by the kernel every second.
//  IMPORTANT::::::Use try_lock() NOT lock() — blocking inside a signal
//       handler can deadlock if the interrupted thread held the lock.
// Read about how to set a value of an AtomicBool in Rust to set TIMED_OUT to false.
// ----------------------------------------------------------
extern "C" fn SigalrmHandler(_sig: libc::c_int) {
    // Hint: if TIMED_OUT.load(Ordering::SeqCst) {}
    let Some(arc) = STATE.get() else { return; };
    let Ok(mut game) = arc.try_lock() else { return; };

    todo!("TODO 3-B: SIGALRM handler — decrement timer, set round_over")
}

// ----------------------------------------------------------
// TODO 4-A: Implement `TimeOutForLevel`
//   Returns the number of seconds allowed per word at each level:
// ----------------------------------------------------------
pub fn TimeOutForLevel(level: usize) -> u64 {
    todo!("TODO 4-A: return seconds per level")
}

// ----------------------------------------------------------
// TODO 3-D: Implement `start`
//   Arm an interval timer that fires SIGALRM every 1 second.
//   Reset TIMED_OUT to false before arming.
// ----------------------------------------------------------
pub fn Start(_timeout_secs: u64) {
    TIMED_OUT.store(false, Ordering::SeqCst);
    todo!("TODO 3-D: arm periodic SIGALRM via libc::setitimer(ITIMER_REAL, 1s/1s)")
}


pub fn Stop() {
    let zero = libc::itimerval {
        it_interval: libc::timeval { tv_sec: 0, tv_usec: 0 },
        it_value:    libc::timeval { tv_sec: 0, tv_usec: 0 },
    };
    unsafe {
        libc::setitimer(libc::ITIMER_REAL, &zero, std::ptr::null_mut());
    }
    TIMED_OUT.store(false, Ordering::SeqCst);
}


