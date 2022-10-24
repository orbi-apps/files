export interface FileTemplate {
    prefix?: string
    suffix?: string
    content?: string
    mimeType: string
}

interface FilesTemplates {
    [key: string]: FileTemplate
}


export default {
    directory: { mimeType: "directory"},
    emptyFile: { mimeType: "text/plain"},
    markdown: { suffix: ".md", mimeType: "text/markdown"},
    python: { suffix: ".py", mimeType: "text/x-python"},
    csv: { suffix: ".csv", mimeType: "text/csv"},
} as FilesTemplates