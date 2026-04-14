--- Configure environment variables for the SDK
--- @param ctx table Context information
--- @field ctx.path string SDK installation directory
--- @return table Environment variable configurations
function PLUGIN:EnvKeys(ctx)
    local mainPath = ctx.path
    return {
        {
            key = "PATH",
            value = mainPath,
        },
    }
end
