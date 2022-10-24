import type { GenericObject } from "./object"

export interface FileFilters {
    foldersOnly: boolean,
    dotFilesHidden: boolean,
    bySuffix: string[]
}

export default (files: GenericObject[], filters: FileFilters) => {
    return files.filter(file => {
        if (filters.foldersOnly && !foldersOnly(file)) return false
        if (filters.dotFilesHidden && !dotFilesHidden(file)) return false
        if (filters.bySuffix.length > 0 && !filterBySuffix(file, filters.bySuffix)) return false

        return true
    })
}

const foldersOnly = (file: GenericObject) => {
    return file.id.mimeType.isFolder()
}

const dotFilesHidden = (file: GenericObject) => {
    return file.name[0] !== "."
}

const filterBySuffix = (file: GenericObject, suffixes: string[]) => {
    for (const extension of suffixes) {
        if(file.name.endsWith(extension)) return true
    }

    return false
}