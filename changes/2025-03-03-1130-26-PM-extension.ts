/* eslint-disable no-unused-vars */
import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import { execSync } from 'child_process';
import { Buffer } from 'node:buffer';
import { GitHubService } from './services/githubService';
import { GitService } from './services/gitService';
import { Tracker } from './services/tracker';
import { SummaryGenerator } from './services/summaryGenerator';
import { Scheduler } from './services/scheduler';
import { ChangeAnalyzer } from './services/changeAnalyzer';
import { platform, homedir } from 'os';

// Interfaces
interface PersistedAuthState {
  username?: string;
  repoName?: string;
  lastWorkspace?: string;
}

interface DevTrackServices {
  outputChannel: vscode.OutputChannel;
  githubService: GitHubService;
  gitService: GitService;
  tracker: Tracker;
  summaryGenerator: SummaryGenerator;
  scheduler: Scheduler | null;
  trackingStatusBar: vscode.StatusBarItem;
  authStatusBar: vscode.StatusBarItem;
  extensionContext: vscode.ExtensionContext;
  changeAnalyzer: ChangeAnalyzer;
}

// Git Installation Handler
class GitInstallationHandler {
  private static readonly DOWNLOAD_URLS = {
    win32: 'https://git-scm.com/download/win',
    darwin: 'https://git-scm.com/download/mac',
    linux: 'https://git-scm.com/download/linux',
  };

  static async checkGitInstallation(
    outputChannel: vscode.OutputChannel
  ): Promise<boolean> {
    try {
      const gitVersion = execSync('git --version', { encoding: 'utf8' });
      outputChannel.appendLine(`DevTrack: Git found - ${gitVersion.trim()}`);
      return true;
    } catch {
... (truncated for brevity)