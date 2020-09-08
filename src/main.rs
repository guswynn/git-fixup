// Copyright 2018 Brandon W Maister
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;

use structopt::StructOpt;

const UPSTREAM_VAR: &str = "GIT_INSTAFIX_UPSTREAM";

#[derive(StructOpt, Debug)]
#[structopt(
    about = "Fix a commit in your history with your currently-staged changes",
    long_about = "Fix a commit in your history with your currently-staged changes

When run with no arguments this will:

  * If you have no staged changes, ask if you'd like to stage all changes
  * Print a `diff --stat` of your currently staged changes
  * Provide a list of commits to fixup or amend going back to:
      * The merge-base of HEAD and the environment var GIT_INSTAFIX_UPSTREAM
        (if it is set)
      * HEAD's upstream
  * Fixup your selected commit with the staged changes
",
    max_term_width = 100,
    setting = structopt::clap::AppSettings::UnifiedHelpMessage,
    setting = structopt::clap::AppSettings::ColoredHelp,
)]
struct Args {
    /// Use `squash!`: change the commit message that you amend
    #[structopt(short = "s", long = "squash")]
    squash: bool,
    /// The maximum number of commits to show when looking for your merge point
    #[structopt(short = "m", long = "max-commits", default_value = "15")]
    max_commits: usize,

    /// Specify a commit to ammend by the subject line of the commit
    #[structopt(short = "P", long)]
    commit_message_pattern: Option<String>,

    #[structopt(long, env(UPSTREAM_VAR))]
    default_upstream_branch: Option<String>,
}

fn main() {
    let mut args = Args::from_args();
    if env::args().next().unwrap().ends_with("squash") {
        args.squash = true
    }
    if let Err(e) = git_fixup::instafix(
        args.squash,
        args.max_commits,
        args.commit_message_pattern,
        args.default_upstream_branch.as_deref(),
    ) {
        // An empty message means don't display any error message
        let msg = e.to_string();
        if !msg.is_empty() {
            println!("Error: {:#}", e);
        }
        std::process::exit(1);
    }
}
