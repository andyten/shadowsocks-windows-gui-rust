
use std::io::Write;

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("res/shadowsocks.ico");
    res.set_language(0x0409); // LANG_ENGLISH

    res.set_manifest(r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
    <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
        <security>
            <requestedPrivileges>
                <requestedExecutionLevel level="asInvoker" uiAccess="false"/>
            </requestedPrivileges>
        </security>
    </trustInfo>

    <assemblyIdentity
        version="1.2.3.4"
        processorArchitecture="*"
        name="Free.Software"
        type="win32"
    />
    <description>Your application description here.</description>
    <dependency>
        <dependentAssembly>
            <assemblyIdentity
                type="win32"
                name="Microsoft.Windows.Common-Controls"
                version="6.0.0.0"
                processorArchitecture="*"
                publicKeyToken="6595b64144ccf1df"
                language="*"
            />
        </dependentAssembly>
    </dependency>
</assembly>
    "#);

    match res.compile() {
        Err(e) => {
            write!(std::io::stderr(), "{}", e).unwrap();
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}
