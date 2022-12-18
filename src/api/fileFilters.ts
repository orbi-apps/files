import type { MimeType, GenericObject } from "./object"

export interface FileFilters {
    foldersOnly: boolean,
    dotFilesHidden: boolean,
    symlinksHidden: boolean,
    bySuffix: string[]
}

export default (files: GenericObject[], filters: FileFilters) => {
    return files.filter(file => {
        if (filters.foldersOnly && !foldersOnly(file)) return false
        if (filters.dotFilesHidden && !dotFilesHidden(file)) return false
        if (filters.symlinksHidden && isSymlink(file)) return false
        if (filters.bySuffix.length > 0 && !filterBySuffix(file, filters.bySuffix)) return false

        return file.name !== '.thinkdrive.container'
    })
}

const foldersOnly = (file: GenericObject) => {
    return file.id.mimeType.isFolder()
}

const isSymlink = (file: GenericObject) => {
    return file.id.mimeType.toString() == "symlink"
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

export enum SortType {
    ModifiedAt = "modified_at",
    Name = "name",
    Size = "size"
}

export const sortBy = (files: GenericObject[], sortType: SortType, ascending = true) => {
    return files.sort((a,b) => {
        if (a.id.mimeType.isFolder() && !b.id.mimeType.isFolder()) return  -1
        if (!a.id.mimeType.isFolder() && b.id.mimeType.isFolder()) return 1
        if (a[sortType] < b[sortType]) return ascending ? -1 : 1
        if (a[sortType] > b[sortType]) return ascending ? 1 : -1
        return 0
    })
}