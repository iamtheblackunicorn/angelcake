# ANGELCAKE :angel: :cake:

***A tasty way to execute build tasks.*** :angel: :cake:

## ABOUT :books:

***ANGELCAKE*** is a small tool I wrote to make executing build routines easy for me. I've been using ***CMake*** for a long time but I wanted a simpler alternative in a syntax I could understand. ***ANGELCAKE*** is that alternative. :heart_on_fire:

## INSTALLATION :inbox_tray:

Make sure that you have the following tools installed and available from the command line:

- [Rust](https://www.rust-lang.org/tools/install)
- [Git](https://git-scm.com/downloads)

One you have that done, you can simply run this command from the command line:

```bash
$ cargo install --git https://github.com/iamtheblackunicorn/angelcake
```

To check that the installation succeeded, run this command:

```bash
$ angelcake -v
```

## USAGE :hammer:

To use ***ANGELCAKE***, you will need to do two steps:

- Create a `Cakefile` with build rules.
- Instruct ***ANGELCAKE*** which build rules to run.

### Create a `Cakefile`

***ANGELCAKE*** uses the same syntax as Angel Markup. Here's an example:

```text
% This is a comment.
'greet' => 'echo Hello World!'
```

Save this into a file called `Cakefile`.

### Run the `greet` routine

Change into the same directory as the `Cakefile` and run this command:

```bash
$ angelcake -r greet
# OR
$ angelcake --runr greet
```

You should see this output:

```text
Hello World!
```

## NOTE :scroll:

- *ANGELCAKE :angel: :cake:* by *Alexander Abraham* a.k.a. *The Black Unicorn :unicorn: :black_heart:* a.k.a. *Angeldust Duke :unicorn: :fire: :pill:*.
- Licensed under the MIT license.
