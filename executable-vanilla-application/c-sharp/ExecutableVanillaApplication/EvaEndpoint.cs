using ApplicationManagement;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using static ApplicationManagement.State.Types;

namespace ExecutableVanillaApplication;

public sealed class EvaEndpoint
{
	private readonly Dictionary<string, (bool IsRunning, State State)> _tasks = new();
	private readonly EvaPipe _pipe;

	public EvaEndpoint()
	{
		_pipe = new EvaPipe($@"\\.\pipe\magi-workspace-manager-{Process.GetCurrentProcess().Id}");
	}

	public ManagerMessage Read()
	{

	}

	public static bool IsWorkspaceManagerRunning()
	{
		return File.Exists($@"\\.\pipe\magi-workspace-manager");
	}

	public void UpdateTaskStatus(string taskId, bool isRunning)
	{
		var state = new State();
		state.Info.Add("");

		_tasks[taskId] = (isRunning, state);

		if (_pipe.IsConnected)
		{
			_pipe.Encode(new ApplicationManagement.Local.ApplicationToManagerMessage
			{
				TaskStateUpdate = new TaskStateUpdate
				{
					TaskId = taskId,
					IsRunning = isRunning,
					State = state
				}
			});
		}
	}
}
