// Copyright 2022 Ethan Kinnear
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use assert_cmd::Command;

fn cmd() -> Command {
    Command::cargo_bin("xshe").unwrap()
}

#[test]
fn test_help_on_no_args() {
    cmd().assert().failure().code(2).stdout("");
}

#[test]
fn test_help_flag() {
    [cmd().arg("-h"), cmd().arg("--help")].map(|x| x.assert().success().code(0).stderr(""));
}

#[test]
fn test_option_text() {
    cmd()
        .arg("bash")
        .args(&["-t", r"IT_WORKS = 'yes'"])
        .assert()
        .success()
        .stdout(concat!(r#"export IT_WORKS="yes";"#, "\n"));
}
