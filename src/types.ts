export interface Tag {
    id: string;
    name: string;
    is_key: boolean;
}

export interface Agent {
    id: string;
    name: string;
    avatar_url: string;
    tags: string[];
}