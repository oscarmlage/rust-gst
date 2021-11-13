# Gst

Gst is a cli command that consumes a remote API to achieve different actions about time-stamps

## What we're looking for

We're looking for a cli command that acts like this:

```sh
$ gst projects [pr]
  - List of projects
$ gst tasks [t] [--project|-pr] 11
  - List of tasks, if -pr grouped per projects else just last open tasks
$ gst start -t 212
  - Add new stamp to the task 212
$ gst stop
  - Stop the last open stamp
$ gst description "Task description"
  - Add description to last stamp
$ gst addtask [--project|-pr] 11 "Task title"
  - Add a task to a project
$ gst status [--last]
  - List last stamps and status (open, closed...), like a summary
```

## Why Rust?

Because my mind wants to learn something new. Also, why not?
