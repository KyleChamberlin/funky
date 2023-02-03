Funky
=====

Do you ever find yourself scrolling back in history to find _that_ command.
You know the one. the one you carfeully crafted a week ago that has all the
right options and switches to invoke the right incantation. Well, Funky can
help you make that bespoke cli invocation into a shell function that is easily
(and repeatedly) available.

Basic Usage
-----------

```sh
funky new my-long-cmd -- long command "with some payload" --and -s -o -m -e --switches "and an $ENV_VAR"
```

Now you will have a zsh function my-long-cmd that you can call directly in your shell.

```sh
my-long-cmd
```

### But, My Command needs to change over time

So, your command needs arguments. You have some component in your command that changes from invocation to invocation, no problem!
Funcky provides a few was to handle this. First, you can simply use `$ENV_VAR`s to inject these pieces, but that can be clunky and
it can be easy to forget which vars need to be set to what values for your desired behavior. For this, we provide an interactive token
selection that allows you to take any token in your command and make it an argument to your function.

How Does It Work?
-----------------

Funky works by leveraging your existing shell's function mechanisms. Funky can update your shell configuration to automatically pick up
new and changed functions at the time of invocation in the shell. This means it is fast, and doesn't require you to re-launch your shell
each time you want to create a new function with Funky.

Simply run `Funky init` to update your preferred shell configuration file.