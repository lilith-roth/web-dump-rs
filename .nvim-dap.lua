local dap = require("dap")

dap.adapters.lldb = {
    type = "executable",
    command = "/usr/bin/codelldb",
    name = "lldb",
}

dap.configurations.rust = {
    {
        name = "web-dump-rs",
        type = "lldb",
        request = "launch",
        program = function()
            return vim.fn.getcwd() .. "/target/debug/web-dump-rs"
        end,
        cwd = "${workspaceFolder}",
        stopOnEntry = false,
        args = { '--wordlist-path', '/usr/share/seclists/Discovery/Web-Content/common.txt', '--target-url', 'http://127.0.0.1:8000' };
    },
}
