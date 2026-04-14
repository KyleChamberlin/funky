--- Return all available versions provided by this plugin
--- @param ctx table Context
--- @return table Available versions
function PLUGIN:Available(ctx)
    local http = require("http")
    local json = require("json")

    local resp, err = http.get({
        url = "https://api.github.com/repos/KyleChamberlin/funky/releases"
    })
    if err ~= nil or resp.status_code ~= 200 then
        return {}
    end

    local releases = json.decode(resp.body)
    local versions = {}
    for _, release in ipairs(releases) do
        if not release.prerelease and not release.draft then
            local tag = release.tag_name or ""
            local version = tag:gsub("^v", "")
            if version ~= "" then
                table.insert(versions, {
                    version = version,
                    note = release.name or "",
                })
            end
        end
    end
    return versions
end
