# `gext-cli`

This is an example project that shows how one might use the `git2ext` API.

## Running

From the `gext-cli`'s root run the following command

```sh
$ cargo run -- /path/to/git/repo/root relative/file/path
```

where `/path/to/git/repo/root` is a git project but *not* the `.git/` directory
and `relative/file/path` is a repo-root-relative filepath to a path tracked by git.
