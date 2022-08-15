import * as vscode from "vscode";

const extensionName = "octave-lsp";

export default class Config implements vscode.WorkspaceConfiguration{
    private _config: vscode.WorkspaceConfiguration;

    constructor(extCtx: vscode.ExtensionContext) {
        this._config = vscode.workspace.getConfiguration(extensionName);

        extCtx.subscriptions.push(vscode.workspace.onDidChangeConfiguration(() => {
            this._config = vscode.workspace.getConfiguration(extensionName);
        }));
    }

    public get<T>(section: string): T | undefined;
    public get<T>(section: string, defaultValue: T): T;
    public get<T>(section: string, defaultValue?: T) {
        return this._config.get<T>(section) ?? defaultValue;
    }
    public has(section: string) {
        return this._config.has(section);
    }
    public inspect<T>(section: string): { key: string; defaultValue?: T; globalValue?: T; workspaceValue?: T; workspaceFolderValue?: T; defaultLanguageValue?: T; globalLanguageValue?: T; workspaceLanguageValue?: T; workspaceFolderLanguageValue?: T; languageIds?: string[];
    } | undefined {
        return this._config.inspect<T>(section);
    }

    public update(section: string, value: any, configurationTarget?: vscode.ConfigurationTarget): Thenable<void> {
        return this._config.update(section, value, configurationTarget);
    }
}

