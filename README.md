# Gst

Gst is a cli command that consumes a remote API to achieve different actions about time-stamps

## What we're looking for

We're looking for a cli command that acts like this:

```sh
$ gst projects
  - List of projects
$ gst tasks [--project|-p] 11 [--last|-l]
  - List of tasks, if -pr grouped per projects else just last open tasks
$ gst stamps [--project|-p] [--last|-l]
  - List last stamps and status (open, closed...), like a summary
$ gst addtask [--project|-p] 11 -t "Task title"
  - Add a task to a project
$ gst stamp --start -t 212 -d "stamp description"
  - Add new stamp to the task 212
$ gst stamp --stop
  - Stop the last open stamp
$ gst stamp --description "Task description"
  - Add description to last stamp
```

## Why Rust?

Because my mind wants to learn something new. Also, why not?
