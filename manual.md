# SahneSH User Manual

1. Introduction to SahneSH
SahneSH is a powerful, user-friendly, and modern command-line shell developed for the Sahne64 Microkernel operating system. Designed with a focus on simplicity and efficiency, SahneSH leverages the Sahne64 System Call API for direct resource and task management, offering a unique set of built-in commands and editor-like keyboard shortcuts.

1.1 Key Features
* Custom Path Separator (∣): SahneSH uses the pipe symbol (∣, U+2223) as the default directory separator, replacing the traditional / or \.

* Built-in Resource Management: Core commands directly interface with the Sahne64 kernel's resource API for file/directory manipulation, status control, and task management.

* Fish-like User Experience: Includes common editor shortcuts (Cut, Copy, Paste) for enhanced command-line editing.

* Raw Mode Input: Operates in terminal raw mode to capture and process every keystroke and complex control sequences.

2. Shell Navigation and Syntax

2.1 The Path Separator (∣)
All file and directory paths in SahneSH must use the pipe symbol (∣) as the separator.

| Concept | SahneSH Syntax | Traditional Equivalent |
| :--- | :--- | :--- |
| Root Directory | root∣ | / or C:\ |
| Relative Path | bin∣app | bin/app |
| Current Directory | Implicitly used or . | . |

2.2 Argument Quoting
SahneSH supports quoting arguments using double quotes ("...") to ensure that arguments containing spaces or special characters (including the path separator ∣) are treated as a single token.

| Example Command | Arguments Passed to Program | Notes |
| :--- | :--- | :--- |
| ls my∣file.txt | ls, my∣file.txt | Correctly parses the single path. |
| ls "My Folder∣File Name" | ls, My Folder∣File Name | Preserves the space within the path. |

3. Built-in Commands Reference
SahneSH implements several commands internally for system and file management, directly using the Sahne64 System Call interface.

| Command | Description | System Call Interaction |
| :--- | :--- | :--- |
| exit | Gracefully terminates the SahneSH shell task. | SYSCALL_TASK_EXIT(0) |
| shutdown | Alias for exit. Terminates the shell. | SYSCALL_TASK_EXIT(0) |
| cd <path> | Changes the current working directory (CWD). | SYSCALL_RESOURCE_ACQUIRE (validation) |
| ls / dir | Lists the contents of the current or specified directory. | SYSCALL_RESOURCE_ACQUIRE, SYSCALL_RESOURCE_READ |
| copy <src> <dest> | Copies the file contents from source to destination. | ACQUIRE (read/write), READ, WRITE, RELEASE |
| paste <src> <dest> | Alias for copy. | Same as copy |
| delete <path> | Permanently deletes a file or directory. | SYSCALL_RESOURCE_CONTROL (with RESOURCE_CONTROL_DELETE) |
| open_the_file <path> | Reads and prints the content of a file to STDOUT. | SYSCALL_RESOURCE_ACQUIRE, SYSCALL_RESOURCE_READ |
| sleep <ms> | Pauses the execution of the shell for the specified duration. | SYSCALL_TASK_SLEEP |
| save | Triggers a system/session state save operation. | SYSCALL_RESOURCE_CONTROL (with RESOURCE_CONTROL_SAVE_STATE) |
| data_recovery | Initiates a simulated data recovery process. | Simulation |
| irrecoverable_deletion | Alias for delete, emphasizing the permanent nature. | Same as delete |
| restart / reset | Attempts to restart the system or the current shell task. | SYSCALL_TASK_EXIT(1) (Signals kernel for restart) |

4. Command Line Editing Shortcuts
SahneSH supports common editing key combinations to enhance productivity within the command buffer. Note that Copy/Cut/Paste use an internal shell clipboard, not the OS clipboard.

| Shortcut | Action | Type | Notes |
| :--- | :--- | :--- | :--- |
| Ctrl + C | Copy | Editor | Copies the current command line content to the internal clipboard. |
| Ctrl + X | Cut | Editor | Copies content and clears the line. |
| Ctrl + V | Paste | Editor | Pastes content from the internal clipboard. |
| Ctrl + Z | Undo | Editor | Placeholder: Indicates the need for history/undo state. |
| Ctrl + Y | Redo | Editor | Placeholder: Indicates the need for history/redo state. |
| Ctrl + A | Select All | Editor | Currently implemented as "Go to start of line" or simple select. |
| Ctrl + S | Save | Built-in | Executes the save command. |
| Ctrl + Shift + L | Insert Separator | Custom | Inserts the custom path separator character: ∣. |
| Ctrl + F | Find | Built-in | Simulates execution of a find/search command. |
| Ctrl + Q | Quit | Built-in | Executes the exit command. |
| Ctrl + P | Print | Built-in | Simulates execution of the open_the_file command. |
| Enter (\n/\r) | Execute | Shell Control | Submits the current command for execution. |
| Backspace/Delete | Delete | Editor | Deletes the character to the left of the cursor. |
