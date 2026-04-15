$WshShell = New-Object -ComObject WScript.Shell
$DesktopPath = [System.Environment]::GetFolderPath('Desktop')

# Define paths for the new shortcut target, working directory, and icon
$targetPath = "c:\Users\Prashant\.gemini\antigravity\scratch\streamdeck\src-tauri\target\release\streamdeck_bin.exe"
$workingDirectory = "c:\Users\Prashant\.gemini\antigravity\scratch\streamdeck\src-tauri\target\release"
$iconPath = "c:\Users\Prashant\.gemini\antigravity\scratch\streamdeck\src-tauri\icons\icon.ico"
$shortcutPath = [System.IO.Path]::Combine($DesktopPath, "StreamDeck.lnk")

$Shortcut = $WshShell.CreateShortcut($shortcutPath)
$Shortcut.TargetPath = $targetPath
# Arguments are no longer needed as we are directly launching the executable
$Shortcut.Arguments = ""
$Shortcut.WorkingDirectory = $workingDirectory
$Shortcut.IconLocation = $iconPath
$Shortcut.WindowStyle = 1 # Normal
$Shortcut.Save()

Write-Host "Shortcut created on Desktop: $shortcutPath"
