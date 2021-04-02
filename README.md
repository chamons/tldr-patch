# tldr-patch

A utility to summarize changes in a PR, ignoring generated files.


## Example

Files modified:
```
tldr-patch.exe https://github.com/Azure/autorest.csharp/pull/1109
eng/Generate.ps1
samples/Azure.AI.DocumentTranslation/Azure.AI.DocumentTranslation.csproj
samples/Azure.AI.DocumentTranslation/readme.md
src/AutoRest.CSharp/AutoRest/Plugins/LowLevelTarget.cs
src/AutoRest.CSharp/Generation/Types/TypeFactory.cs
src/AutoRest.CSharp/Generation/Writers/LowLevelClientWriter.cs
src/AutoRest.CSharp/Generation/Writers/RequestWriterHelpers.cs
src/AutoRest.CSharp/Output/Models/LowLevelRestClient.cs
src/AutoRest.CSharp/Output/Models/RestClientBuilder.cs
src/AutoRest.CSharp/Output/Models/Types/DataPlaneOutputLibrary.cs
src/AutoRest.CSharp/Output/Models/Types/LowLevelOutputLibrary.cs
src/AutoRest.CSharp/Output/Models/Types/MgmtOutputLibrary.cs
src/AutoRest.CSharp/Output/Models/Types/OutputLibrary.cs
src/AutoRest.CSharp/Output/Models/Types/ResourceTypeBuilder.cs
src/AutoRest.CSharp/Properties/launchSettings.json
```

Full Patch:
```
tldr-patch.exe -p https://github.com/Azure/autorest.csharp/pull/1109
--- a/eng/Generate.ps1
+++ b/eng/Generate.ps1
@@ -111,7 +111,8 @@ $projectNames =
     'Azure.Storage.Tables',
     'Azure.ResourceManager.Sample',
     'Azure.Management.Storage',
-    'Azure.Network.Management.Interface'
+    'Azure.Network.Management.Interface',
+    'Azure.AI.DocumentTranslation'
...
```

## Arguments
```
tldr-patch.exe --help
tldr-patch 0.1
Chris Hamons <chris.hamons@gmail.com>

USAGE:
    tldr-patch.exe [FLAGS] <url>

ARGS:
    <url>    PR url to parse e.g. https://github.com/A/B/pull/1

FLAGS:
    -h, --help       Prints help information
    -p, --patch      Instead of showing filed edited, show actual diffs
    -V, --version    Prints version information
```
