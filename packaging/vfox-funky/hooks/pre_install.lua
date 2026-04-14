--- Returns pre-installation information
--- @param ctx table
--- @field ctx.version string User-input version
--- @return table Version information
function PLUGIN:PreInstall(ctx)
    local version = ctx.version

    local os_type = RUNTIME.osType
    local arch_type = RUNTIME.archType

    local os_name
    if os_type == "darwin" then
        os_name = "macos"
    elseif os_type == "linux" then
        os_name = "linux"
    elseif os_type == "windows" then
        os_name = "windows"
    else
        error("Unsupported OS: " .. os_type)
    end

    local arch_name
    if arch_type == "amd64" then
        arch_name = "x64"
    elseif arch_type == "arm64" then
        arch_name = "aarch64"
    elseif arch_type == "386" then
        arch_name = "x86"
    else
        error("Unsupported architecture: " .. arch_type)
    end

    local url = "https://github.com/KyleChamberlin/funky/releases/download/v"
        .. version
        .. "/funky-v"
        .. version
        .. "-"
        .. os_name
        .. "-"
        .. arch_name
        .. ".zip"

    return {
        version = version,
        url = url,
    }
end
