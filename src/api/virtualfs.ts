import { listFolderContent, move_to } from "@junction"
import { MimeType, ObjectId, type GenericObject } from "./object"
import type { ProviderId } from "./Providers"
import { create, del, listProviders, open, rename } from "@junction"
import _ from "lodash"
import fileFilters, { type FileFilters } from "./fileFilters"

export class VirtualFS {
    #activeProvider: ProviderId | undefined
    #path: Path = newPath()
    #files: GenericObject[] = []
    #onChangeCallbacks: (() => void)[] = []
    #onSelectionChangeCallbacks: (() => void)[] = []
    #providers: ProviderId[]
    #history: History = new History()
    #selectedFiles: number[] = []
    #selectionLocked = false
    fileFilters: FileFilters = {
        foldersOnly: false,
        dotFilesHidden: true,
        symlinksHidden: true,
        bySuffix: []
    }

    constructor() {
        this.fetchProviders()
        this.#history.push({path: this.#path, provider: this.#activeProvider})
    }

    register(fn: () => void) {
        this.#onChangeCallbacks.push(fn)
    }

    onSelectionChange(fn: () => void) {
        this.#onSelectionChangeCallbacks.push(fn)
    }

    get files() {
        return this.#files
    }

    get selectedFilesIndices() {
        return this.#selectedFiles
    }

    get selectedFiles() {
        return this.#selectedFiles.map(i => this.#files[i])
    }

    get providers() {
        return this.#providers
    }

    get activeProvider() {
        return this.#activeProvider
    }

    async create(file: GenericObject) {
        create(this.#path.top().objectId, this.#activeProvider, file)
        this.fetchFilesAndFolders()
    }

    async open() {
        for (const index of this.#selectedFiles) {
            open(this.#files[index].id, this.#activeProvider)
        }
        this.#selectedFiles = []
    }

    async openInBrowser() {
    }

    async download() {
    }

    async rename(from: string, to: string) {
        for (const index of this.#selectedFiles) {
            const newName = from.replaceAll(from, to)
            rename(this.#files[index].id, this.#activeProvider, newName)
        }
        this.#selectedFiles = []
        this.fetchFilesAndFolders()
    }

    async move(newParent: GenericObject, newProvider: ProviderId) {
        console.log(this.selectedFiles)
        for (const file of this.selectedFiles) {
            move_to(file.id, this.#activeProvider, newParent.id, newProvider)
        }

        this.#selectedFiles = []
        this.fetchFilesAndFolders()
    }

    async delete() {
        for (const index of this.#selectedFiles) {
            del(this.#files[index].id, this.#activeProvider)
        }
        this.#selectedFiles = []
        this.fetchFilesAndFolders()
    }

    async setProvider(provider: ProviderId) {
        this.#activeProvider = provider
        this.#history.push({path: this.#path, provider: this.#activeProvider})
        this.fetch()
    }

    async push(objectId: ObjectId, name: string) : Promise<void> {
        this.#path.push({ objectId, name })
        this.#history.push({path: this.#path, provider: this.#activeProvider})
        await this.fetch()
    }

    async pop() : Promise<PathElement> {
        const value = this.#path.pop()
        this.#history.push({path: this.#path, provider: this.#activeProvider})
        await this.fetch()
        return value;
    }

    async setPath(path: Path) : Promise<void> {
        this.#path.set(path)
        this.#history.push({path: this.#path, provider: this.#activeProvider})
        await this.fetch()
    }

    pathAsArray() : PathElement[] {
        if (!this.#activeProvider) {return []}

        return [{objectId: new ObjectId(this.#activeProvider.id, new MimeType("directory")), name: this.#activeProvider.id},...this.#path.asArray()]
    }

    async fetch() {
        if (!this.#activeProvider) {
            this.fetchProviders()
        } else {
            this.fetchFilesAndFolders()
        }
    }

    async fetchFilesAndFolders() : Promise<void> {
        const z = (x): GenericObject => { return {id : new ObjectId(x.id, new MimeType(x.mime_type)), name: x.name}}
        const objects = (await listFolderContent(this.#path.top().objectId, this.#activeProvider))
            .map(x => z(x))

        this.#files = fileFilters(objects, this.fileFilters).filter(
            (x) => x.name !== '.thinkdrive.container'
        ).sort((a,b) => {
            if (a.id.mimeType.isFolder() && !b.id.mimeType.isFolder()) return -1
            if (!a.id.mimeType.isFolder() && b.id.mimeType.isFolder()) return 1
            let fa = a.name.toLowerCase()
            let fb = b.name.toLowerCase()
            if (fa < fb) {
                return -1
            }
            if (fa > fb) {
                return 1
            }
            return 0
        })

        this.#selectedFiles = []

        this.#onChangeCallbacks.forEach(fn => {
            fn()
        })
    }

    async fetchProviders() : Promise<void> {
        this.#providers = (await listProviders()).map(x => {
            return {
                id: x.id,
                type: x.provider_type,
                data: x.data
            }
        })

        this.#onChangeCallbacks.forEach(fn => {
            fn()
        })
    }

    goBack() {
        if (!this.#activeProvider) return
        const state = this.#history.goBack()
        if (state) {
            this.#path = state.path
            this.#activeProvider = state.provider
            this.fetch()
        }
    }

    goForward() {
        const state = this.#history.goForward()
        if (state) {
            this.#path = state.path
            this.#activeProvider = state.provider
            this.fetch()
        }
    }

    gotoIndex(index: number) {
        if (index == 0) {
            this.#activeProvider = undefined
            const path = newPath()
            this.setPath(path)
        } else {
            const path = newPath(this.#path._path.slice(0, index - 1))
            this.setPath(path)
        }
    }
    
    selectFile(index: number) {
        if (this.#selectionLocked) return

        this.#selectedFiles.push(index)
        this.#onSelectionChangeCallbacks.forEach(fn => {
            fn()
        })
    }
    
    clearSelection() {
        if (this.#selectionLocked) return

        this.#selectedFiles = []
        this.#onSelectionChangeCallbacks.forEach(fn => {
            fn()
        })
    }

    lockSelection() {
        this.#selectionLocked = true
    }

    unlockSelection() {
        this.#selectionLocked = false
    }
}

interface PathElement {
    objectId: ObjectId
    name: string
}

export interface Path {
    _path: PathElement[]
    push: (element: PathElement) => void
    pop: () => PathElement
    top: () => PathElement
    set: (path: Path) => void
    asArray: () => PathElement[]
    toIdString: () => string
}

const newPath = (elements: PathElement[] = []) => {
    return {
        _path: elements,
    
        push(element: PathElement) : void {
            this._path.push(element)
        },

        pop() : PathElement {
            return this._path.pop()
        },

        top() : PathElement {
            if (this._path.length > 0) {
                return this._path[this._path.length - 1]
            } else {
                return {
                    objectId: ObjectId.directory(""),
                    name: ""
                }
            }
        },

        set(path: Path) : void {
            this._path = path.asArray()
        },

        asArray() : PathElement[] {
            return this._path
        },

        toIdString() : string {
            const pathArray = this._path.map((x: PathElement) => {
                return x.objectId.toString()
            })
            return pathArray.join("/")
        }
    }
}

export class History {
    #stateHistory: any[] = []
    #nextCache: any[] = []

    goBack() {
        const state = this.#stateHistory.pop()
        if (state) this.#nextCache.push(state)
        return _.cloneDeep(this.#stateHistory[this.#stateHistory.length - 1])
    }

    goForward() {
        const state = this.#nextCache.pop()
        if (state) this.#stateHistory.push(state)
        return _.cloneDeep(this.#stateHistory[this.#stateHistory.length - 1])
    }

    push(state: any) {
        this.#stateHistory.push(_.cloneDeep(state))
        this.#nextCache = []
    }
}