--- Execute additional operations after install
--- @param ctx table
--- @field ctx.rootPath string SDK installation root path
function PLUGIN:PostInstall(ctx)
    local rootPath = ctx.rootPath
    local mainSdkInfo = ctx.sdkInfo[PLUGIN.name]
    if mainSdkInfo == nil then
        return
    end
    local path = mainSdkInfo.path

    -- Set executable permission on non-Windows
    if RUNTIME.osType ~= "windows" then
        local binary = path .. "/funky"
        os.execute("chmod +x " .. binary)
    end
end
