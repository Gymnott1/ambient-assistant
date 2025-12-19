#!/usr/bin/env python3
import tkinter as tk
from tkinter import ttk
import subprocess
import time
import threading
import requests
import json

class AmbientAssistant:
    def __init__(self):
        self.root = tk.Tk()
        self.root.title("Ambient Assistant")
        self.root.geometry("320x400")
        self.root.configure(bg='#1e1e2e')
        self.root.overrideredirect(True)  # Remove window decorations
        
        # Set transparency and always on top
        self.root.attributes('-alpha', 0.85)  # More transparency
        self.is_pinned = True
        self.root.attributes('-topmost', True)
        
        self.setup_ui()
        self.setup_bindings()
        
    def setup_ui(self):
        # Main container with rounded appearance
        main_frame = tk.Frame(self.root, bg='', relief='flat', bd=0)
        main_frame.pack(fill='both', expand=True, padx=2, pady=2)
        
        # Header (drag handle) with gradient-like effect
        header = tk.Frame(main_frame, bg='#1a1a2e', height=40, relief='flat', bd=0)
        header.pack(fill='x')
        header.pack_propagate(False)
        
        title_label = tk.Label(header, text="⠿ Ambient Assistant", 
                              bg='#1a1a2e', fg='#89b4fa', font=('Arial', 10, 'bold'))
        title_label.pack(side='left', padx=12, pady=10)
        
        # Minimize button
        minimize_btn = tk.Button(header, text="−", bg='#1a1a2e', fg='#89b4fa', 
                               font=('Arial', 12, 'bold'), bd=0, padx=8, pady=2,
                               command=self.toggle_minimize, activebackground='#45475a')
        minimize_btn.pack(side='right', padx=5, pady=10)
        
        self.status_label = tk.Label(header, text="📌 Always on Top", 
                                   bg='#1a1a2e', fg='#6c7086', font=('Arial', 8))
        self.status_label.pack(side='right', padx=10, pady=10)
        
        # Make header draggable
        header.bind('<Button-1>', self.start_drag)
        header.bind('<B1-Motion>', self.drag_window)
        title_label.bind('<Button-1>', self.start_drag)
        title_label.bind('<B1-Motion>', self.drag_window)
        
        # Content area with better spacing
        self.content = tk.Frame(main_frame, bg='')
        self.content.pack(fill='both', expand=True, padx=15, pady=15)
        self.content_visible = True
        self.minimized = False
        
        # Live indicator
        self.live_indicator = tk.Label(header, text="●", bg='#1a1a2e', fg='#f38ba8', font=('Arial', 8))
        self.live_indicator.pack(side='right', padx=(0,5), pady=10)
        
        # Start fetching live data
        self.suggestion_widgets = []
        self.start_live_updates()
            
    def create_suggestion_item(self, parent, title, subtitle, command=""):
        # Container with subtle border
        container = tk.Frame(parent, bg='', pady=1)
        container.pack(fill='x', pady=2)
        
        frame = tk.Frame(container, bg='#1a1a2e', relief='flat', bd=0)
        frame.pack(fill='x', padx=1)
        
        title_label = tk.Label(frame, text=title, bg='#1a1a2e', fg='#f5e0dc', 
                              font=('Arial', 9, 'bold'), anchor='w')
        title_label.pack(fill='x', padx=12, pady=(10,2))
        
        subtitle_label = tk.Label(frame, text=subtitle, bg='#1a1a2e', fg='#a6adc8', 
                                 font=('Arial', 8), anchor='w')
        subtitle_label.pack(fill='x', padx=12, pady=(0,10))
        
        # Enhanced hover effects
        def on_enter(e):
            frame.configure(bg='#313244')
            title_label.configure(bg='#313244', fg='#89b4fa')
            subtitle_label.configure(bg='#313244', fg='#cdd6f4')
            
        def on_leave(e):
            frame.configure(bg='#1a1a2e')
            title_label.configure(bg='#1a1a2e', fg='#f5e0dc')
            subtitle_label.configure(bg='#1a1a2e', fg='#a6adc8')
            
        def on_click(e):
            print(f"🚀 Executing: {title} | Command: {command}")
            # Flash effect
            frame.configure(bg='#89b4fa')
            self.root.after(100, lambda: on_leave(None))
            
        for widget in [frame, title_label, subtitle_label]:
            widget.bind('<Enter>', on_enter)
            widget.bind('<Leave>', on_leave)
            widget.bind('<Button-1>', on_click)
            
        return container
        
    def start_live_updates(self):
        def fetch_data():
            while True:
                try:
                    response = requests.get('http://localhost:8080/suggestions', timeout=2)
                    if response.status_code == 200:
                        data = response.json()
                        self.root.after(0, lambda: self.update_suggestions(data['suggestions']))
                        self.root.after(0, self.animate_live_indicator)
                except Exception as e:
                    print(f"❌ Backend connection failed: {e}")
                    # Use fallback data
                    fallback = [
                        {"suggestion": "🔌 Backend Offline", "command": "reconnect", "comment": "Trying to reconnect..."},
                        {"suggestion": "🎵 Play Focus Music", "command": "spotify", "comment": "Offline mode"},
                        {"suggestion": "📝 Git: Commit Changes", "command": "git", "comment": "Local changes"},
                        {"suggestion": "🧹 Cleanup Downloads", "command": "cleanup", "comment": "1.2GB temp files"},
                        {"suggestion": "🚀 Deploy App", "command": "deploy", "comment": "Ready to deploy"},
                    ]
                    self.root.after(0, lambda: self.update_suggestions(fallback))
                time.sleep(3)
                
        thread = threading.Thread(target=fetch_data, daemon=True)
        thread.start()
        
    def update_suggestions(self, suggestions):
        # Clear existing suggestions
        for widget in self.suggestion_widgets:
            widget.destroy()
        self.suggestion_widgets.clear()
        
        # Add new suggestions
        for item in suggestions:
            widget = self.create_suggestion_item(
                self.content, 
                item['suggestion'], 
                item['comment'], 
                item['command']
            )
            self.suggestion_widgets.append(widget)
            
    def animate_live_indicator(self):
        colors = ['#f38ba8', '#fab387', '#f9e2af', '#a6e3a1', '#89b4fa']
        current_color = self.live_indicator.cget('fg')
        try:
            current_index = colors.index(current_color)
            next_color = colors[(current_index + 1) % len(colors)]
        except ValueError:
            next_color = colors[0]
        self.live_indicator.config(fg=next_color)
    
    def setup_bindings(self):
        # Right-click menu
        self.root.bind('<Button-3>', self.show_context_menu)
        
        # Keyboard shortcuts
        self.root.bind('<Control-p>', self.toggle_pin)
        self.root.bind('<Control-h>', self.toggle_hide)
        self.root.bind('<Control-m>', lambda e: self.toggle_minimize())
        self.root.focus_set()
        
    def show_context_menu(self, event):
        menu = tk.Menu(self.root, tearoff=0, bg='#1a1a2e', fg='#f5e0dc', activebackground='#313244')
        menu.add_command(label="Toggle Always on Top", command=self.toggle_pin)
        menu.add_command(label="Toggle Hide Content", command=self.toggle_hide)
        menu.add_command(label="Toggle Minimize", command=self.toggle_minimize)
        menu.tk_popup(event.x_root, event.y_root)
        
    def toggle_pin(self, event=None):
        self.is_pinned = not self.is_pinned
        
        if self.is_pinned:
            self.root.attributes('-topmost', True)
            self.status_label.config(text="📌 Always on Top")
            print("📌 Always on top: ENABLED")
        else:
            self.root.attributes('-topmost', False)
            # Force window to go below using system command
            subprocess.run(['wmctrl', '-r', 'Ambient Assistant', '-b', 'remove,above'], 
                         capture_output=True)
            subprocess.run(['wmctrl', '-r', 'Ambient Assistant', '-b', 'add,below'], 
                         capture_output=True)
            self.root.after(100, lambda: subprocess.run(['wmctrl', '-r', 'Ambient Assistant', '-b', 'remove,below'], 
                                                       capture_output=True))
            self.status_label.config(text="Ctrl+P to Pin")
            print("📍 Always on top: DISABLED")
            
    def toggle_hide(self, event=None):
        self.content_visible = not self.content_visible
        
        if self.content_visible:
            self.content.pack(fill='both', expand=True, padx=15, pady=15)
            print("👁️ Content shown")
        else:
            self.content.pack_forget()
            print("🙈 Content hidden")
            
    def toggle_minimize(self):
        self.minimized = not self.minimized
        
        if self.minimized:
            self.content.pack_forget()
            self.root.geometry("320x40")  # Just header height
            print("📦 Minimized")
        else:
            self.content.pack(fill='both', expand=True, padx=15, pady=15)
            self.root.geometry("320x400")  # Full height
            print("📋 Expanded")
        
    def start_drag(self, event):
        self.drag_start_x = event.x
        self.drag_start_y = event.y
        
    def drag_window(self, event):
        x = self.root.winfo_pointerx() - self.drag_start_x
        y = self.root.winfo_pointery() - self.drag_start_y
        self.root.geometry(f"+{x}+{y}")
        
    def run(self):
        print("📌 Ambient Assistant started with always-on-top enabled")
        print("🔗 Connecting to backend at http://localhost:8080/suggestions")
        self.root.mainloop()

if __name__ == "__main__":
    app = AmbientAssistant()
    app.run()