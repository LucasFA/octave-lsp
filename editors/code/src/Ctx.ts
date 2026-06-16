import * as vscode from "vscode";
import * as path from "path";

import Config from "./Config";
import * as lang_client from "vscode-languageclient/node";

function getPlatformTarget(): string {
    const arch = process.arch === "arm64" ? "aarch64" : "x86_64";
    switch (process.platform) {
        case "linux": return `${arch}-unknown-linux-gnu`;
        case "darwin": return `${arch}-apple-darwin`;
        case "win32": return `${arch}-pc-windows-msvc`;
        default: throw new Error(`Unsupported platform: ${process.platform}`);
    }
}

export default class Ctx implements vscode.Disposable {
    private constructor(
        private readonly extCtx: vscode.ExtensionContext,
        readonly config: Config,
        readonly client: lang_client.LanguageClient,
        readonly serverPath: string
    ) { }

    static async create(extCtx: vscode.ExtensionContext): Promise<Ctx> {
        const config = new Config(extCtx);
        const serverBaseName = "octave-lsp";
        const serverExt = process.platform === "win32" ? ".exe" : "";

        let serverPath: string | null = config.get("server.path") ?? null;
        if (!serverPath) {
            serverPath = path.join(
                extCtx.extensionPath, "server", getPlatformTarget(),
                serverBaseName + serverExt
            );
        }

        const serverOptions: lang_client.ServerOptions = {
            run: { command: serverPath },
            debug: {
                command: serverPath,
                args: ["--repl"],
            },
        };

        const clientOptions: lang_client.LanguageClientOptions = {
            documentSelector: [{ scheme: "file", language: "octave" }],
            outputChannelName: "Octave Language Server",
            outputChannel: vscode.window.createOutputChannel("Octave Language Server"),
            revealOutputChannelOn: lang_client.RevealOutputChannelOn.Error,
        };

        const client = new lang_client.LanguageClient(
            "Octave Language Server", serverOptions, clientOptions
        );

        const ctx = new Ctx(extCtx, config, client, serverPath);
        ctx.extCtx.subscriptions.push(ctx);
        await ctx.client.start();
        return ctx;
    }

    public push(d: vscode.Disposable): void {
        this.extCtx.subscriptions.push(d);
    }

    public dispose() {
        return this.client.stop();
    }
}

export type Cmd = (...args: unknown[]) => unknown;
