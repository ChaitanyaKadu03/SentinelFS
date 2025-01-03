1. Overview
The CLI File Manager is a Command Line Interface (CLI) application written in Rust. It allows users to interact with files on their local system by supporting operations like reading file contents, writing text to files, listing files in a directory, deleting files, and—most notably—monitoring a directory for changes in real-time.

2. Features
Read File: Outputs the contents of a specified file to the terminal.
Write to File: Allows users to write or append text data to a file.
List Directory: Lists all files in a specified directory.
Delete File: Deletes specified files (with a confirmation prompt).
Monitor Directory (New / Stand-out Feature):
Watches a directory in real-time, printing a notification whenever files are created, modified, or deleted.
Automatically reloads any file list or index if desired.
Help Command: Provides usage information about available commands and flags.

⚠️⚠️⚠️ @todo- There is a need to well comment the codebase for better developer experience.