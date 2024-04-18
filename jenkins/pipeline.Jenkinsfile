pipeline {

    environment {
        SONAR_PROJECT_KEY = "AlexeyNavalniyPrinted_avel_00c2ac06-035e-4591-964e-229da28aa9fd"
    }

    agent {
        kubernetes {
            label 'docker-build-pod'
            yaml """
            apiVersion: v1
            kind: Pod
            spec:
              containers:
                - name: docker
                  image: docker:dind
                  command: [ 'sleep' ]
                  args: [ 'infinity' ]
                  volumeMounts:
                    - mountPath: /var/run/docker.sock
                      name: docker-sock
              volumes:
                - name: docker-sock
                  hostPath:
                    path: /var/run/docker.sock
            """
        }
    }

    stages {
        stage('Pull git repository') {
            steps {
                checkout([
                    $class: 'GitSCM',
                    branches: [[name: '*/main']],
                    extensions: [],
                    userRemoteConfigs: [[url: 'https://github.com/AlexeyNavalniyPrinted/avel']]
                ])
            }
        }

        stage('Static code analysis') {
            steps {
                script {
                    def scannerHome = tool 'SonarScanner'
                    withSonarQubeEnv(installationName: 'sonarqube') {
                        sh "${scannerHome}/bin/sonar-scanner -Dsonar.projectKey=${env.SONAR_PROJECT_KEY}"
                    }
                }
            }
        }

        stage('Build Image') {
            steps {
                container('docker') {
                    sh 'docker build -t alex23451234/avel:latest .'
                }
            }
        }

        stage('Push Image') {
            steps {
                container('docker') {
                    withCredentials([usernamePassword(credentialsId: 'dockerhub-credentials', usernameVariable: 'USERNAME', passwordVariable: 'PASSWORD')]) {
                        sh 'docker login -u $USERNAME --password-stdin $PASSWORD'
                        sh 'docker push alex23451234/avel:latest'
                    }
                }
            }
        }
    }
}

