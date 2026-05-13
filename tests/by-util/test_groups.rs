// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

//spell-checker: ignore coreutil

use uutests::new_ucmd;
use uutests::unwrap_or_return;
use uutests::util::{TestScenario, check_coreutil_version, expected_result, whoami};
use uutests::util_name;

const VERSION_MIN_MULTIPLE_USERS: &str = "8.31"; // this feature was introduced in GNU's coreutils 8.31

#[test]
#[cfg(unix)]
fn test_invalid_arg() {
    new_ucmd!().arg("--definitely-invalid").fails_with_code(1);
}

#[test]
#[cfg(unix)]
fn test_groups() {
    let ts = TestScenario::new(util_name!());
    let result = ts.ucmd().succeeds();
    let exp_result = unwrap_or_return!(expected_result(&ts, &[]));

    result
        .stdout_is(exp_result.stdout_str())
        .stderr_is(exp_result.stderr_str())
        .code_is(exp_result.code());
}

#[test]
#[cfg(unix)]
fn test_groups_username() {
    let test_users = [&whoami()[..]];

    let ts = TestScenario::new(util_name!());
    let result = ts.ucmd().args(&test_users).succeeds();
    let exp_result = unwrap_or_return!(expected_result(&ts, &test_users));

    result
        .stdout_is(exp_result.stdout_str())
        .stderr_is(exp_result.stderr_str())
        .code_is(exp_result.code());
}

#[test]
#[cfg(unix)]
fn test_groups_username_multiple() {
    unwrap_or_return!(check_coreutil_version(
        util_name!(),
        VERSION_MIN_MULTIPLE_USERS
    ));
    let test_users = ["root", "man", "postfix", "sshd", &whoami()];

    let ts = TestScenario::new(util_name!());
    let result = ts.ucmd().args(&test_users).fails();
    let exp_result = unwrap_or_return!(expected_result(&ts, &test_users));

    result
        .stdout_is(exp_result.stdout_str())
        .stderr_is(exp_result.stderr_str())
        .code_is(exp_result.code());
}

#[test]
#[cfg(target_os = "linux")]
fn test_groups_duplicate_groups() {
    use std::process::Command;

    use uutests::{get_tests_binary, util::unshare_bin};

    let unshare = unwrap_or_return!(unshare_bin());
    let unshare_args = ["-U", "--map-groups=auto", "--"];

    let exp_result = Command::new(&unshare)
        .args(unshare_args)
        .arg(util_name!())
        .output()
        .unwrap();

    let result = Command::new(&unshare)
        .args(unshare_args)
        .arg(get_tests_binary!())
        .arg(util_name!())
        .output()
        .unwrap();

    assert_eq!(result.status.code(), exp_result.status.code());

    let exp_stdout = String::from_utf8_lossy(&exp_result.stdout);
    let act_stdout = String::from_utf8_lossy(&result.stdout);
    assert_eq!(act_stdout, exp_stdout);
}
