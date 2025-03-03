/* eslint-disable no-unused-vars */
import { Buffer } from 'buffer';
import * as vscode from 'vscode';
import simpleGit, { SimpleGit, SimpleGitOptions } from 'simple-git';
import * as path from 'path';
import { EventEmitter } from 'events';
import { OutputChannel } from 'vscode';
import { promisify } from 'util';
import { exec } from 'child_process';
import { execSync } from 'child_process';
import process from 'process';
const execAsync = promisify(exec);
import * as fs from 'fs';
import { fileURLToPath } from 'url';

interface TimeDistribution {
  hour: number;
  changes: number;
}

interface ActivityTimelineEntry {
  date: string;
  commits: number;
  filesChanged: number;
}

interface FileTypeStats {
  name: string;
  count: number;
  percentage: number;
}

interface DevTrackStats {
  totalTime: number;
  filesModified: number;
  totalCommits: number;
  linesChanged: number;
  activityTimeline: ActivityTimelineEntry[];
  timeDistribution: TimeDistribution[];
  fileTypes: FileTypeStats[];
}

interface GitServiceEvents {
  commit: (message: string) => void;
  error: (error: Error) => void;
  'operation:start': (operation: string) => void;
  'operation:end': (operation: string) => void;
  retry: (operation: string, attempt: number) => void;
  push: (branch: string) => void;
}
... (truncated for brevity)