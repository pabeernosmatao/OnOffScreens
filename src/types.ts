export interface Screen {
  uuid: string;
  name: string;
  number: number | null;
  connected: boolean;
  enabled: boolean;
  x: number;
  y: number;
  width: number;
  height: number;
  scale: number;
  primary: boolean;
}

export interface Profile {
  id: string;
  name: string;
  screens: string[];
}

export interface Settings {
  tray: boolean;
  lastProfile: string | null;
  lang: "auto" | "es" | "ca" | "en";
}

export interface AppState {
  screens: Screen[];
  profiles: Profile[];
  activeProfileId: string | null;
  settings: Settings;
  autostart: boolean;
  trayRunning: boolean;
  lang: "es" | "ca" | "en";
  backendError: string | null;
  version: string;
  configPath: string;
  mapPath: string;
  configDir: string;
}

export interface ProfileInput {
  id?: string;
  name: string;
  screens: string[];
}

export type Tab = "profiles" | "options";
