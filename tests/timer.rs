use hangman_rs::timer::{RegisterSignalHandler, Start, Stop, TimeOutForLevel, TIMED_OUT};
use hangman_rs::game::GameState;
use std::sync::{Arc, Mutex, OnceLock};
use std::sync::atomic::Ordering;

#[test]
fn test_timeout_level1() {
    assert_eq!(TimeOutForLevel(1), 300);
}

#[test]
fn test_timeout_level2() {
    assert_eq!(TimeOutForLevel(2), 180);
}

#[test]
fn test_timeout_level3() {
    assert_eq!(TimeOutForLevel(3), 120);
}

// ----------------------------------------------------------
// Signal handler tests
//
// STATE in timer.rs is a OnceLock — only the first call to
// RegisterSignalHandler actually stores the Arc. Both tests
// must share the same Arc and run serially (TEST_LOCK ensures
// they don't interleave and corrupt secs_remaining).
// ----------------------------------------------------------

static TEST_STATE: OnceLock<Arc<Mutex<GameState>>> = OnceLock::new();
static TEST_LOCK: Mutex<()> = Mutex::new(());

fn test_state() -> Arc<Mutex<GameState>> {
    TEST_STATE.get_or_init(|| {
        let s = Arc::new(Mutex::new(GameState::new()));
        RegisterSignalHandler(Arc::clone(&s));
        s
    }).clone()
}

// Validates RegisterSignalHandler: raise SIGALRM manually and
// check the handler decremented secs_remaining exactly once.
#[test]
fn test_signal_handler_registered() {
    let _guard = TEST_LOCK.lock().unwrap();
    let state = test_state();
    state.lock().unwrap().NewRound("test".to_string(), 30, "test".to_string(), 1, 1);
    TIMED_OUT.store(false, Ordering::SeqCst);

    unsafe { libc::raise(libc::SIGALRM); }
    std::thread::sleep(std::time::Duration::from_millis(100));

    let secs = state.lock().unwrap().secs_remaining;
    assert_eq!(secs, 29, "Handler must decrement secs_remaining on SIGALRM: got {}", secs);
}

// Validates Start/setitimer: arm the timer and wait 2.5 s —
// the handler must have fired at least twice via the kernel timer.
#[test]
fn test_start_fires_handler_twice() {
    let _guard = TEST_LOCK.lock().unwrap();
    let state = test_state();
    state.lock().unwrap().NewRound("test".to_string(), 30, "test".to_string(), 1, 1);
    TIMED_OUT.store(false, Ordering::SeqCst);

    Start(30);
    std::thread::sleep(std::time::Duration::from_millis(2500));
    Stop();

    let secs = state.lock().unwrap().secs_remaining;
    assert!(secs <= 28, "Start must arm setitimer: handler fired at least twice, secs = {}", secs);
}
