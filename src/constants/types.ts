export interface ButtonFlags {
  [code: string]: string;
}

export interface Packets {
  [name: string]: string;
}

export interface Drivers {
  [index: number]: Driver;
}

export interface Tyres {
  [index: number]: Tyre;
}

export interface Tyre {
  color: string;
  name: string;
}

export interface Track {
  name: string;
}

export interface Driver {
  abbreviation: string;
  firstName: string;
  lastName: string;
}

export interface EventCodes {
  [name: string]: string;
}

export interface Teams {
  [index: number]: Team;
}

export interface Team {
  name: string;
  color: string;
}
