import platform
import os

class WindowingSystem:
    def __init__(self, name: str, os_name: str):
        self.name = name
        self.os_name = os_name

def get_window_system() -> WindowingSystem:
    os_name = platform.system()
    windowing_system = WindowingSystem("Unknown", os_name)

    if os_name == "Linux":
        command = os.popen("echo $XDG_SESSION_TYPE")
        ws_name = command.read().strip()
        if ws_name == "wayland":
            windowing_system.name = "Wayland"
        elif ws_name == "x11":
            windowing_system.name = "X11"
    return windowing_system
